#!/usr/bin/env python3
"""Builds assets/assets.db from assets/BinData/*.json atomically and streams table-by-table to keep RAM low."""

import json
import math
import os
import sqlite3
import sys
import time

SCHEMA_VERSION = "1"
SPATIAL_CELL_SIZE = 5000.0

SPAWNABLE_ITEM_TYPES = {
    "AirWallEffectTrigger", "AirPassage", "AnnunciatorCenter", "AnnunciatorWire",
    "BattleVehicle", "BurstCrystalFoundation", "CableWayHook", "ChallengeInteract",
    "Collect", "Collect2", "Collect3", "CollectAnimal", "CollectAnimalPart",
    "CollectWithHook", "CollectWithListener", "ConveyorBelt",
    "CurveControlDestructible", "DeadeyeHookTrigger", "Destructible",
    "DestructibleControl", "DestructibleExploreInteractor", "DestructibleWithHook",
    "Disc", "DungeonEntry", "DungeonEntryWithTrigger", "DynamicPortalCreater",
    "ExploreSkillGroupInteractor", "ExploreSkillInteractor", "FireBoxLamp",
    "GameplayInteractGear", "GravityFlip", "HookLockPoint", "HookTreasureBox",
    "HookWithRange", "InteractGear", "InteractGear2", "InteractGearCanHit",
    "InteractGearGroup", "InteractGearTrigger", "InteractiveConditionListener",
    "JigsawItem", "KiteHook", "LevelQteTrigger", "Lift", "Lift2",
    "MoveNoRenderPortal", "NoRenderPortal", "NoRenderPortalWithInteract",
    "Portal", "PullingObject", "RangeTriggerTargetGear", "Resurrection",
    "RollingFireball", "SceneAura", "SequenceLift", "SimpleInteract",
    "StateSceneItem", "StateTrigger", "TargetGear", "TargetGear2",
    "TargetGearExploreInteractor", "TargetGearGroup", "TargetGearGroup2",
    "TargetGearPro", "TargetGearWithConditionListener",
    "TargetGearWithLevelPrefabPerform", "TargetGearWithVariable",
    "TeleControl", "TeleControl3", "TeleControl4", "TeleControlGroup",
    "Teleporter", "TreasureBox", "VehicleSceneItem", "VisionTreasureBox",
    "WaterSpout",
}

TREASURE_OR_DESTRUCTIBLE_TYPES = {
    "TreasureBox", "HookTreasureBox", "VisionTreasureBox",
    "Destructible", "DestructibleWithHook", "DestructibleExploreInteractor",
}

KV_INT_TABLES = [
    ("Damage", "Id"),
    ("BaseProperty", "Id"),
    ("TrailPhantomProp", "Id"),
    ("ModelConfigPreload", "Id"),
    ("Condition", "Id"),
    ("ConditionGroup", "Id"),
    ("PassiveSkill", "Id"),
    ("DropPackage", "Id"),
    ("DropGroup", "UnitId"),
]

KV_STR_TABLES = [
    ("Flow", "Id"),
    ("AiStateMachineConfig", "Id"),
    ("FlowState", "StateKey"),
]

DEDICATED_HANDLED_PREFIXES = (
    "LevelEntityConfig_", "LevelEntityConfig.",
    "TemplateConfig.", "Buff.", "EntitySkillPreload.",
    "QuestNodeData.", "LevelPlayNodeData.",
    "FlowState.", "Flow.", "AiStateMachineConfig.",
    "Damage.", "BaseProperty.", "TrailPhantomProp.",
    "ModelConfigPreload.", "Condition.", "ConditionGroup.",
    "PassiveSkill.", "DropPackage.", "DropGroup.",
)


def cell_coord(coord: float):
    if not math.isfinite(coord):
        return None
    scaled = math.floor(coord / SPATIAL_CELL_SIZE)
    if scaled < -1_000_000.0 or scaled > 1_000_000.0:
        return None
    return int(scaled)


def is_spawnable_blueprint_name(bp: str) -> bool:
    return (
        "Monster" in bp
        or "Animal" in bp
        or "npc" in bp.lower()
        or "Treasure" in bp
        or "Teleport" in bp
        or "AirWall" in bp
    )


def compute_spawnable(bp_type: str, bp_map: dict, tpl_reward_flags: dict) -> int:
    bp = bp_map.get(bp_type)
    if bp is not None:
        logic, etype = bp
        if logic in ("Monster", "Animal", "Npc", "Vision", "Vehicle"):
            return 1
        if logic == "Item":
            if etype in SPAWNABLE_ITEM_TYPES:
                return 1
            has_rw_or_tb = tpl_reward_flags.get(bp_type)
            if has_rw_or_tb is not None:
                has_rw, has_tb = has_rw_or_tb
                if etype in TREASURE_OR_DESTRUCTIBLE_TYPES:
                    return 1 if (has_rw or has_tb) else 0
                if has_rw and "Treasure" in bp_type:
                    return 1
            return 0
        return 0
    return 1 if is_spawnable_blueprint_name(bp_type) else 0


def collect_bindata_stamps(bindata_dir: str) -> dict[str, tuple[int, int]]:
    stamps = {}
    for entry in os.scandir(bindata_dir):
        if not entry.is_file():
            continue
        name = entry.name
        if name.startswith("._") or not name.endswith(".json"):
            continue
        st = entry.stat()
        stamps[name[:-5]] = (int(st.st_mtime_ns), int(st.st_size))
    return stamps


def is_db_fresh(db_path: str, bindata_dir: str) -> bool:
    if not os.path.exists(db_path) or os.path.getsize(db_path) < 4096:
        return False
    try:
        conn = sqlite3.connect(f"file:{db_path}?mode=ro", uri=True)
        cur = conn.cursor()
        cur.execute("SELECT v FROM _meta WHERE k='schema_version'")
        row = cur.fetchone()
        if not row or row[0] != SCHEMA_VERSION:
            conn.close()
            return False
        cur.execute("SELECT name, mtime_ns, size FROM _file_stamps")
        db_stamps = {r[0]: (int(r[1]), int(r[2])) for r in cur.fetchall()}
        conn.close()
        cur_stamps = collect_bindata_stamps(bindata_dir)
        return db_stamps == cur_stamps
    except Exception:
        return False


def build_db(bindata_dir: str, out_path: str) -> None:
    t0 = time.time()
    tmp_path = out_path + ".tmp"
    if os.path.exists(tmp_path):
        os.remove(tmp_path)

    conn = sqlite3.connect(tmp_path)
    conn.execute("PRAGMA journal_mode=OFF")
    conn.execute("PRAGMA synchronous=OFF")
    conn.execute("PRAGMA temp_store=MEMORY")
    conn.execute("PRAGMA cache_size=-32768")

    conn.executescript("""
        CREATE TABLE _meta (k TEXT PRIMARY KEY, v TEXT NOT NULL);
        CREATE TABLE _file_stamps (name TEXT PRIMARY KEY, mtime_ns INTEGER NOT NULL, size INTEGER NOT NULL);

        CREATE TABLE level_entities (
            idx INTEGER PRIMARY KEY,
            entity_id INTEGER NOT NULL,
            map_id INTEGER NOT NULL,
            area_id INTEGER NOT NULL,
            blueprint_type TEXT NOT NULL,
            in_sleep INTEGER NOT NULL,
            is_hidden INTEGER NOT NULL,
            cell_x INTEGER,
            cell_y INTEGER,
            spawnable INTEGER NOT NULL,
            exploratory_degree INTEGER NOT NULL,
            is_sun_spirit INTEGER NOT NULL,
            data TEXT NOT NULL
        );

        CREATE TABLE template_configs (
            id INTEGER PRIMARY KEY,
            idx INTEGER NOT NULL,
            blueprint_type TEXT NOT NULL,
            is_passenger INTEGER NOT NULL,
            data TEXT NOT NULL
        );

        CREATE TABLE buffs (
            id INTEGER PRIMARY KEY,
            id_prefix INTEGER NOT NULL,
            is_infinite INTEGER NOT NULL,
            data TEXT NOT NULL
        );

        CREATE TABLE entity_skill_preload (
            idx INTEGER PRIMARY KEY,
            id INTEGER NOT NULL,
            actor_blueprint TEXT NOT NULL,
            skill_id INTEGER NOT NULL,
            skill_prefix INTEGER NOT NULL,
            data TEXT NOT NULL
        );

        CREATE TABLE kv_int (
            tbl TEXT NOT NULL,
            key_id INTEGER NOT NULL,
            data TEXT NOT NULL,
            PRIMARY KEY (tbl, key_id)
        );

        CREATE TABLE kv_str (
            tbl TEXT NOT NULL,
            key_str TEXT NOT NULL,
            data TEXT NOT NULL,
            PRIMARY KEY (tbl, key_str)
        );

        CREATE TABLE node_trees (
            kind INTEGER NOT NULL,
            owner_id INTEGER NOT NULL,
            seq INTEGER NOT NULL,
            data TEXT NOT NULL,
            PRIMARY KEY (kind, owner_id, seq)
        );

        CREATE TABLE raw_files (
            name TEXT PRIMARY KEY,
            data TEXT NOT NULL
        );
    """)

    # 1. BlueprintConfig helper map + TemplateConfig import
    bp_map = {}
    with open(os.path.join(bindata_dir, "BlueprintConfig.json"), "r", encoding="utf-8") as f:
        bp_rows = json.load(f)
    for row in bp_rows:
        bp_type = row.get("BlueprintType") or ""
        if bp_type and bp_type not in bp_map:
            bp_map[bp_type] = (row.get("EntityLogic") or "", row.get("EntityType") or "")
    del bp_rows

    tpl_reward_flags = {}
    with open(os.path.join(bindata_dir, "TemplateConfig.json"), "r", encoding="utf-8") as f:
        tpl_rows = json.load(f)
    tpl_batch = []
    for idx, row in enumerate(tpl_rows):
        bp_type = row.get("BlueprintType") or ""
        comps = row.get("ComponentsData") or {}
        has_rw = "RewardComponent" in comps and comps["RewardComponent"] is not None
        has_tb = "TreasureBoxComponent" in comps and comps["TreasureBoxComponent"] is not None
        if bp_type and bp_type not in tpl_reward_flags:
            tpl_reward_flags[bp_type] = (has_rw, has_tb)
        is_passenger = 1 if ("Passenger" in bp_type) else 0
        tpl_batch.append((int(row["Id"]), idx, bp_type, is_passenger, json.dumps(row, separators=(",", ":"), ensure_ascii=False)))
    conn.executemany("INSERT OR REPLACE INTO template_configs VALUES (?,?,?,?,?)", tpl_batch)
    conn.commit()
    del tpl_rows, tpl_batch

    # 2. LevelEntityConfig_1..N streaming insert
    le_idx = 0
    chunk = 1
    while True:
        path = os.path.join(bindata_dir, f"LevelEntityConfig_{chunk}.json")
        if not os.path.exists(path):
            break
        with open(path, "r", encoding="utf-8") as f:
            rows = json.load(f)
        batch = []
        for row in rows:
            bp_type = row.get("BlueprintType") or ""
            sp_flag = compute_spawnable(bp_type, bp_map, tpl_reward_flags)
            tr = row.get("Transform") or []
            cx = cy = None
            if sp_flag and len(tr) >= 1 and isinstance(tr[0], dict):
                px = float(tr[0].get("X", 0.0)) / 100.0
                py = float(tr[0].get("Y", 0.0)) / 100.0
                cx = cell_coord(px)
                cy = cell_coord(py)
                if cx is None or cy is None:
                    sp_flag = 0
            comps = row.get("ComponentsData") or {}
            bi = comps.get("BaseInfoComponent") or {}
            cat = bi.get("Category") or {}
            exp_deg = int(cat.get("ExploratoryDegree") or 0)
            is_sun = 1 if ("RiLing" in bp_type or "SunSpirit" in bp_type) else 0
            batch.append((
                le_idx,
                int(row.get("EntityId", 0)),
                int(row.get("MapId", 0)),
                int(row.get("AreaId", 0)),
                bp_type,
                1 if row.get("InSleep") else 0,
                1 if row.get("IsHidden") else 0,
                cx,
                cy,
                sp_flag,
                exp_deg,
                is_sun,
                json.dumps(row, separators=(",", ":"), ensure_ascii=False),
            ))
            le_idx += 1
        conn.executemany("INSERT INTO level_entities VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?)", batch)
        conn.commit()
        del rows, batch
        chunk += 1
    del bp_map, tpl_reward_flags

    # 3. Buff.json
    with open(os.path.join(bindata_dir, "Buff.json"), "r", encoding="utf-8") as f:
        buff_rows = json.load(f)
    batch = []
    for row in buff_rows:
        bid = int(row["Id"])
        s = str(bid)
        pref = int(s[:4]) if len(s) > 4 else 0
        is_inf = 1 if (
            row.get("DurationPolicy") == 1
            and not row.get("ApplicationSourceTagRequirements")
            and not row.get("ApplicationTagRequirements")
            and not row.get("PrematureExpirationEffects")
        ) else 0
        batch.append((bid, pref, is_inf, json.dumps(row, separators=(",", ":"), ensure_ascii=False)))
    conn.executemany("INSERT OR REPLACE INTO buffs VALUES (?,?,?,?)", batch)
    conn.commit()
    del buff_rows, batch

    # 4. EntitySkillPreload.json
    with open(os.path.join(bindata_dir, "EntitySkillPreload.json"), "r", encoding="utf-8") as f:
        esp_rows = json.load(f)
    batch = []
    for idx, row in enumerate(esp_rows):
        sid = int(row.get("SkillId", 0))
        sstr = str(sid)
        spref = int(sstr[:4]) if len(sstr) >= 4 else sid
        batch.append((
            idx,
            int(row.get("Id", idx + 1)),
            row.get("ActorBlueprint") or "",
            sid,
            spref,
            json.dumps(row, separators=(",", ":"), ensure_ascii=False),
        ))
    conn.executemany("INSERT INTO entity_skill_preload VALUES (?,?,?,?,?,?)", batch)
    conn.commit()
    del esp_rows, batch

    # 5. KV Int & KV Str tables
    for tbl, kfield in KV_INT_TABLES:
        path = os.path.join(bindata_dir, f"{tbl}.json")
        if not os.path.exists(path):
            continue
        with open(path, "r", encoding="utf-8") as f:
            rows = json.load(f)
        if isinstance(rows, dict):
            rows = list(rows.values())
        batch = [(tbl, int(r[kfield]), json.dumps(r, separators=(",", ":"), ensure_ascii=False)) for r in rows]
        conn.executemany("INSERT OR REPLACE INTO kv_int VALUES (?,?,?)", batch)
        conn.commit()
        del rows, batch

    for tbl, kfield in KV_STR_TABLES:
        path = os.path.join(bindata_dir, f"{tbl}.json")
        if not os.path.exists(path):
            continue
        with open(path, "r", encoding="utf-8") as f:
            rows = json.load(f)
        if isinstance(rows, dict):
            rows = list(rows.values())
        batch = [(tbl, str(r[kfield]), json.dumps(r, separators=(",", ":"), ensure_ascii=False)) for r in rows]
        conn.executemany("INSERT OR REPLACE INTO kv_str VALUES (?,?,?)", batch)
        conn.commit()
        del rows, batch

    # 6. Node Trees (QuestNodeData=0, LevelPlayNodeData=1)
    for kind, fname in ((0, "QuestNodeData.json"), (1, "LevelPlayNodeData.json")):
        path = os.path.join(bindata_dir, fname)
        if not os.path.exists(path):
            continue
        with open(path, "r", encoding="utf-8") as f:
            rows = json.load(f)
        seq_counter = {}
        batch = []
        for entry in rows:
            k = str(entry.get("Key", ""))
            us = k.find("_")
            if us <= 0:
                continue
            try:
                owner_id = int(k[:us])
            except ValueError:
                continue
            seq = seq_counter.get(owner_id, 0)
            seq_counter[owner_id] = seq + 1
            data = entry.get("Data")
            if data is None:
                continue
            batch.append((kind, owner_id, seq, json.dumps(data, separators=(",", ":"), ensure_ascii=False)))
            if len(batch) >= 20000:
                conn.executemany("INSERT INTO node_trees VALUES (?,?,?,?)", batch)
                batch.clear()
        if batch:
            conn.executemany("INSERT INTO node_trees VALUES (?,?,?,?)", batch)
        conn.commit()
        del rows, batch, seq_counter

    # 7. Remaining JSON files stored in raw_files for fast RewardIndex & DataTables loading
    stamps = collect_bindata_stamps(bindata_dir)
    raw_batch = []
    for name in sorted(stamps):
        fname = name + ".json"
        if fname.startswith(DEDICATED_HANDLED_PREFIXES):
            continue
        path = os.path.join(bindata_dir, fname)
        with open(path, "r", encoding="utf-8") as f:
            raw = f.read()
        raw_batch.append((name, raw))
        if len(raw_batch) >= 50:
            conn.executemany("INSERT OR REPLACE INTO raw_files VALUES (?,?)", raw_batch)
            conn.commit()
            raw_batch.clear()
    if raw_batch:
        conn.executemany("INSERT OR REPLACE INTO raw_files VALUES (?,?)", raw_batch)
        conn.commit()

    # 8. Build Indexes & Stamps
    conn.executescript("""
        CREATE INDEX idx_le_map_eid ON level_entities(map_id, entity_id);
        CREATE INDEX idx_le_eid ON level_entities(entity_id);
        CREATE INDEX idx_le_cell ON level_entities(map_id, spawnable, cell_x, cell_y);
        CREATE INDEX idx_le_map_sp ON level_entities(map_id, spawnable);
        CREATE INDEX idx_le_bp ON level_entities(map_id, blueprint_type);
        CREATE INDEX idx_le_area_exp ON level_entities(area_id, exploratory_degree);
        CREATE INDEX idx_le_sun ON level_entities(is_sun_spirit) WHERE is_sun_spirit = 1;

        CREATE INDEX idx_tpl_bp ON template_configs(blueprint_type);
        CREATE INDEX idx_tpl_pass ON template_configs(is_passenger) WHERE is_passenger = 1;

        CREATE INDEX idx_buff_auto ON buffs(id_prefix, is_infinite);

        CREATE INDEX idx_esp_bp ON entity_skill_preload(actor_blueprint);
        CREATE INDEX idx_esp_pref ON entity_skill_preload(skill_prefix);
    """)

    conn.executemany(
        "INSERT INTO _file_stamps VALUES (?,?,?)",
        [(k, v[0], v[1]) for k, v in stamps.items()],
    )
    conn.execute("INSERT INTO _meta VALUES ('schema_version', ?)", (SCHEMA_VERSION,))
    conn.execute("INSERT INTO _meta VALUES ('built_at', ?)", (str(int(time.time())),))
    conn.commit()
    conn.close()

    os.replace(tmp_path, out_path)
    elapsed = time.time() - t0
    sz_mb = os.path.getsize(out_path) / (1024 * 1024)
    print(f"[build_assets_db] Built {out_path} ({sz_mb:.1f} MiB) in {elapsed:.2f}s ({le_idx} level_entities)")


def main() -> int:
    import argparse
    ap = argparse.ArgumentParser()
    ap.add_argument("--bindata", default="assets/BinData")
    ap.add_argument("--out", default="assets/assets.db")
    ap.add_argument("--if-stale", action="store_true")
    args = ap.parse_args()
    if args.if_stale and is_db_fresh(args.out, args.bindata):
        print(f"[build_assets_db] {args.out} is up to date.")
        return 0
    build_db(args.bindata, args.out)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
