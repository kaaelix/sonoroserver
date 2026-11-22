using System;
using System.Collections.Generic;
using System.IO;
using System.Text.Json;
using System.Text.RegularExpressions;
using Microsoft.Data.Sqlite;

namespace AkiLogicService;

public static class SqliteStore
{
    public static readonly string DbPath = Environment.GetEnvironmentVariable("GAMESV_DB_PATH")
        ?? Path.Combine(BinDataCache.RepoRoot, "state", "gamesv.db");

    private static readonly object InitLock = new();
    private static bool _initialized = false;

    public static SqliteConnection OpenConnection()
    {
        EnsureInitialized();
        var conn = new SqliteConnection($"Data Source={DbPath};Mode=ReadWriteCreate;Pooling=True;");
        conn.Open();
        using var pragma = conn.CreateCommand();
        pragma.CommandText = "PRAGMA journal_mode=WAL; PRAGMA synchronous=NORMAL; PRAGMA busy_timeout=5000;";
        pragma.ExecuteNonQuery();
        return conn;
    }

    public static void EnsureInitialized()
    {
        if (_initialized) return;
        lock (InitLock)
        {
            if (_initialized) return;
            var dir = Path.GetDirectoryName(DbPath);
            if (!string.IsNullOrEmpty(dir)) Directory.CreateDirectory(dir);

            using var conn = new SqliteConnection($"Data Source={DbPath};Mode=ReadWriteCreate;Pooling=True;");
            conn.Open();
            using var cmd = conn.CreateCommand();
            cmd.CommandText = @"
PRAGMA journal_mode=WAL;
PRAGMA synchronous=NORMAL;
PRAGMA busy_timeout=5000;

CREATE TABLE IF NOT EXISTS state_files (
    path TEXT PRIMARY KEY,
    content TEXT NOT NULL,
    updated_at INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS players (
    player_id INTEGER PRIMARY KEY,
    name TEXT NOT NULL DEFAULT '',
    level INTEGER NOT NULL DEFAULT 1,
    exp INTEGER NOT NULL DEFAULT 0,
    coin INTEGER NOT NULL DEFAULT 0,
    rare_coin INTEGER NOT NULL DEFAULT 0,
    cash_coin INTEGER NOT NULL DEFAULT 0,
    sex INTEGER NOT NULL DEFAULT 1,
    world_level INTEGER NOT NULL DEFAULT 1,
    updated_at INTEGER NOT NULL DEFAULT 0
);
CREATE TABLE IF NOT EXISTS player_battlepass (
    player_id INTEGER PRIMARY KEY,
    bp_id INTEGER NOT NULL DEFAULT 1,
    level INTEGER NOT NULL DEFAULT 70,
    exp INTEGER NOT NULL DEFAULT 0,
    weekly_exp INTEGER NOT NULL DEFAULT 0,
    pay_status INTEGER NOT NULL DEFAULT 2,
    had_enter INTEGER NOT NULL DEFAULT 1,
    taken_rewards_json TEXT NOT NULL DEFAULT '[]',
    taken_tasks_json TEXT NOT NULL DEFAULT '[]',
    updated_at INTEGER NOT NULL DEFAULT 0
);
CREATE TABLE IF NOT EXISTS player_activity_claims (
    player_id INTEGER NOT NULL,
    claim_key TEXT NOT NULL,
    value_json TEXT NOT NULL DEFAULT '1',
    updated_at INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (player_id, claim_key)
);
CREATE TABLE IF NOT EXISTS player_liveness (
    player_id INTEGER PRIMARY KEY,
    liveness_count INTEGER NOT NULL DEFAULT 100,
    claimed_boxes_json TEXT NOT NULL DEFAULT '[]',
    claimed_tasks_json TEXT NOT NULL DEFAULT '[]',
    updated_at INTEGER NOT NULL DEFAULT 0
);
CREATE TABLE IF NOT EXISTS player_achievements (
    player_id INTEGER NOT NULL,
    achievement_id INTEGER NOT NULL,
    status INTEGER NOT NULL DEFAULT 1,
    progress INTEGER NOT NULL DEFAULT 1,
    is_taken INTEGER NOT NULL DEFAULT 0,
    updated_at INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (player_id, achievement_id)
);
CREATE TABLE IF NOT EXISTS player_shop_bought (
    player_id INTEGER NOT NULL,
    shop_id INTEGER NOT NULL,
    goods_id INTEGER NOT NULL,
    bought_count INTEGER NOT NULL DEFAULT 0,
    updated_at INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (player_id, shop_id, goods_id)
);
CREATE TABLE IF NOT EXISTS player_kv (
    player_id INTEGER NOT NULL,
    ns TEXT NOT NULL,
    k TEXT NOT NULL,
    val TEXT NOT NULL,
    updated_at INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (player_id, ns, k)
);
";
            cmd.ExecuteNonQuery();
            _initialized = true;
            Console.WriteLine($"[SqliteStore] Connected to shared WAL database: {DbPath}");
        }
    }

    public record BattlePassRow(
        int PlayerId,
        int BpId,
        int Level,
        int Exp,
        int WeeklyExp,
        int PayStatus,
        bool HadEnter,
        List<int[]> TakenRewards,
        List<int> TakenTasks
    );

    public static BattlePassRow GetOrCreateBattlePass(int playerId)
    {
        using var conn = OpenConnection();
        using (var cmd = conn.CreateCommand())
        {
            cmd.CommandText = "SELECT bp_id, level, exp, weekly_exp, pay_status, had_enter, taken_rewards_json, taken_tasks_json FROM player_battlepass WHERE player_id = $pid;";
            cmd.Parameters.AddWithValue("$pid", playerId);
            using var r = cmd.ExecuteReader();
            if (r.Read())
            {
                var rewards = JsonSerializer.Deserialize<List<int[]>>(r.GetString(6)) ?? new();
                var tasks = JsonSerializer.Deserialize<List<int>>(r.GetString(7)) ?? new();
                return new BattlePassRow(
                    playerId,
                    r.GetInt32(0),
                    r.GetInt32(1),
                    r.GetInt32(2),
                    r.GetInt32(3),
                    r.GetInt32(4),
                    r.GetInt32(5) != 0,
                    rewards,
                    tasks
                );
            }
        }

        var now = DateTimeOffset.UtcNow.ToUnixTimeSeconds();
        using (var ins = conn.CreateCommand())
        {
            ins.CommandText = @"
INSERT OR IGNORE INTO player_battlepass(player_id, bp_id, level, exp, weekly_exp, pay_status, had_enter, taken_rewards_json, taken_tasks_json, updated_at)
VALUES($pid, 1, 70, 0, 0, 2, 1, '[]', '[]', $ts);";
            ins.Parameters.AddWithValue("$pid", playerId);
            ins.Parameters.AddWithValue("$ts", now);
            ins.ExecuteNonQuery();
        }
        return new BattlePassRow(playerId, 1, 70, 0, 0, 2, true, new(), new());
    }

    public static void SaveBattlePass(BattlePassRow row)
    {
        using var conn = OpenConnection();
        using var cmd = conn.CreateCommand();
        cmd.CommandText = @"
INSERT INTO player_battlepass(player_id, bp_id, level, exp, weekly_exp, pay_status, had_enter, taken_rewards_json, taken_tasks_json, updated_at)
VALUES($pid, $bp, $lvl, $exp, $wexp, $pay, $enter, $rew, $tsk, $ts)
ON CONFLICT(player_id) DO UPDATE SET
    bp_id = excluded.bp_id,
    level = excluded.level,
    exp = excluded.exp,
    weekly_exp = excluded.weekly_exp,
    pay_status = excluded.pay_status,
    had_enter = excluded.had_enter,
    taken_rewards_json = excluded.taken_rewards_json,
    taken_tasks_json = excluded.taken_tasks_json,
    updated_at = excluded.updated_at;";
        cmd.Parameters.AddWithValue("$pid", row.PlayerId);
        cmd.Parameters.AddWithValue("$bp", row.BpId);
        cmd.Parameters.AddWithValue("$lvl", row.Level);
        cmd.Parameters.AddWithValue("$exp", row.Exp);
        cmd.Parameters.AddWithValue("$wexp", row.WeeklyExp);
        cmd.Parameters.AddWithValue("$pay", row.PayStatus);
        cmd.Parameters.AddWithValue("$enter", row.HadEnter ? 1 : 0);
        cmd.Parameters.AddWithValue("$rew", JsonSerializer.Serialize(row.TakenRewards));
        cmd.Parameters.AddWithValue("$tsk", JsonSerializer.Serialize(row.TakenTasks));
        cmd.Parameters.AddWithValue("$ts", DateTimeOffset.UtcNow.ToUnixTimeSeconds());
        cmd.ExecuteNonQuery();
    }

    public record LivenessRow(int PlayerId, int LivenessCount, List<int> ClaimedBoxes, List<int> ClaimedTasks);

    public static LivenessRow GetOrCreateLiveness(int playerId)
    {
        using var conn = OpenConnection();
        using (var cmd = conn.CreateCommand())
        {
            cmd.CommandText = "SELECT liveness_count, claimed_boxes_json, claimed_tasks_json FROM player_liveness WHERE player_id = $pid;";
            cmd.Parameters.AddWithValue("$pid", playerId);
            using var r = cmd.ExecuteReader();
            if (r.Read())
            {
                return new LivenessRow(
                    playerId,
                    r.GetInt32(0),
                    JsonSerializer.Deserialize<List<int>>(r.GetString(1)) ?? new(),
                    JsonSerializer.Deserialize<List<int>>(r.GetString(2)) ?? new()
                );
            }
        }

        using (var ins = conn.CreateCommand())
        {
            ins.CommandText = "INSERT OR IGNORE INTO player_liveness(player_id, liveness_count, claimed_boxes_json, claimed_tasks_json, updated_at) VALUES($pid, 100, '[]', '[]', $ts);";
            ins.Parameters.AddWithValue("$pid", playerId);
            ins.Parameters.AddWithValue("$ts", DateTimeOffset.UtcNow.ToUnixTimeSeconds());
            ins.ExecuteNonQuery();
        }
        return new LivenessRow(playerId, 100, new(), new());
    }

    public static void SaveLiveness(LivenessRow row)
    {
        using var conn = OpenConnection();
        using var cmd = conn.CreateCommand();
        cmd.CommandText = @"
INSERT INTO player_liveness(player_id, liveness_count, claimed_boxes_json, claimed_tasks_json, updated_at)
VALUES($pid, $cnt, $boxes, $tasks, $ts)
ON CONFLICT(player_id) DO UPDATE SET
    liveness_count = excluded.liveness_count,
    claimed_boxes_json = excluded.claimed_boxes_json,
    claimed_tasks_json = excluded.claimed_tasks_json,
    updated_at = excluded.updated_at;";
        cmd.Parameters.AddWithValue("$pid", row.PlayerId);
        cmd.Parameters.AddWithValue("$cnt", row.LivenessCount);
        cmd.Parameters.AddWithValue("$boxes", JsonSerializer.Serialize(row.ClaimedBoxes));
        cmd.Parameters.AddWithValue("$tasks", JsonSerializer.Serialize(row.ClaimedTasks));
        cmd.Parameters.AddWithValue("$ts", DateTimeOffset.UtcNow.ToUnixTimeSeconds());
        cmd.ExecuteNonQuery();
    }

    public static Dictionary<int, int> GetShopBoughtCounts(int playerId, int shopId)
    {
        var result = new Dictionary<int, int>();
        using var conn = OpenConnection();
        using var cmd = conn.CreateCommand();
        cmd.CommandText = "SELECT goods_id, bought_count FROM player_shop_bought WHERE player_id = $pid AND shop_id = $sid;";
        cmd.Parameters.AddWithValue("$pid", playerId);
        cmd.Parameters.AddWithValue("$sid", shopId);
        using var r = cmd.ExecuteReader();
        while (r.Read())
        {
            result[r.GetInt32(0)] = r.GetInt32(1);
        }
        return result;
    }

    public static void IncrementShopBought(int playerId, int shopId, int goodsId, int delta)
    {
        using var conn = OpenConnection();
        using var cmd = conn.CreateCommand();
        cmd.CommandText = @"
INSERT INTO player_shop_bought(player_id, shop_id, goods_id, bought_count, updated_at)
VALUES($pid, $sid, $gid, $delta, $ts)
ON CONFLICT(player_id, shop_id, goods_id) DO UPDATE SET
    bought_count = player_shop_bought.bought_count + excluded.bought_count,
    updated_at = excluded.updated_at;";
        cmd.Parameters.AddWithValue("$pid", playerId);
        cmd.Parameters.AddWithValue("$sid", shopId);
        cmd.Parameters.AddWithValue("$gid", goodsId);
        cmd.Parameters.AddWithValue("$delta", delta);
        cmd.Parameters.AddWithValue("$ts", DateTimeOffset.UtcNow.ToUnixTimeSeconds());
        cmd.ExecuteNonQuery();
    }

    public static Dictionary<int, (int Progress, bool IsTaken)> GetAchievements(int playerId)
    {
        var result = new Dictionary<int, (int, bool)>();
        using var conn = OpenConnection();
        using var cmd = conn.CreateCommand();
        cmd.CommandText = "SELECT achievement_id, progress, is_taken FROM player_achievements WHERE player_id = $pid;";
        cmd.Parameters.AddWithValue("$pid", playerId);
        using var r = cmd.ExecuteReader();
        while (r.Read())
        {
            result[r.GetInt32(0)] = (r.GetInt32(1), r.GetInt32(2) != 0);
        }
        return result;
    }

    public static void MarkAchievementsTaken(int playerId, IEnumerable<int> achievementIds)
    {
        using var conn = OpenConnection();
        using var tx = conn.BeginTransaction();
        var now = DateTimeOffset.UtcNow.ToUnixTimeSeconds();
        foreach (var id in achievementIds)
        {
            using var cmd = conn.CreateCommand();
            cmd.Transaction = tx;
            cmd.CommandText = @"
INSERT INTO player_achievements(player_id, achievement_id, status, progress, is_taken, updated_at)
VALUES($pid, $aid, 1, 1, 1, $ts)
ON CONFLICT(player_id, achievement_id) DO UPDATE SET
    is_taken = 1,
    updated_at = excluded.updated_at;";
            cmd.Parameters.AddWithValue("$pid", playerId);
            cmd.Parameters.AddWithValue("$aid", id);
            cmd.Parameters.AddWithValue("$ts", now);
            cmd.ExecuteNonQuery();
        }
        tx.Commit();
    }

    public static void SetActivityClaim(int playerId, string claimKey, string valueJson)
    {
        using var conn = OpenConnection();
        using var cmd = conn.CreateCommand();
        cmd.CommandText = @"
INSERT INTO player_activity_claims(player_id, claim_key, value_json, updated_at)
VALUES($pid, $k, $v, $ts)
ON CONFLICT(player_id, claim_key) DO UPDATE SET
    value_json = excluded.value_json,
    updated_at = excluded.updated_at;";
        cmd.Parameters.AddWithValue("$pid", playerId);
        cmd.Parameters.AddWithValue("$k", claimKey);
        cmd.Parameters.AddWithValue("$v", valueJson);
        cmd.Parameters.AddWithValue("$ts", DateTimeOffset.UtcNow.ToUnixTimeSeconds());
        cmd.ExecuteNonQuery();
    }

    public static HashSet<string> GetActivityClaims(int playerId)
    {
        var set = new HashSet<string>();
        using var conn = OpenConnection();
        using var cmd = conn.CreateCommand();
        cmd.CommandText = "SELECT claim_key FROM player_activity_claims WHERE player_id = $pid;";
        cmd.Parameters.AddWithValue("$pid", playerId);
        using var r = cmd.ExecuteReader();
        while (r.Read())
        {
            set.Add(r.GetString(0));
        }
        return set;
    }

    public static void FinishAchievement(int playerId, int achievementId, int progress = 1)
    {
        using var conn = OpenConnection();
        using var cmd = conn.CreateCommand();
        cmd.CommandText = @"
INSERT INTO player_achievements(player_id, achievement_id, status, progress, is_taken, updated_at)
VALUES($pid, $aid, 1, $prog, 0, $ts)
ON CONFLICT(player_id, achievement_id) DO UPDATE SET
    status = 1,
    progress = MAX(player_achievements.progress, excluded.progress),
    updated_at = excluded.updated_at;";
        cmd.Parameters.AddWithValue("$pid", playerId);
        cmd.Parameters.AddWithValue("$aid", achievementId);
        cmd.Parameters.AddWithValue("$prog", progress);
        cmd.Parameters.AddWithValue("$ts", DateTimeOffset.UtcNow.ToUnixTimeSeconds());
        cmd.ExecuteNonQuery();
    }

    public static string? GetKv(int playerId, string ns, string key)
    {
        using var conn = OpenConnection();
        using var cmd = conn.CreateCommand();
        cmd.CommandText = "SELECT val FROM player_kv WHERE player_id = $pid AND ns = $ns AND k = $k;";
        cmd.Parameters.AddWithValue("$pid", playerId);
        cmd.Parameters.AddWithValue("$ns", ns);
        cmd.Parameters.AddWithValue("$k", key);
        return cmd.ExecuteScalar() as string;
    }

    public static void SetKv(int playerId, string ns, string key, string value)
    {
        using var conn = OpenConnection();
        using var cmd = conn.CreateCommand();
        cmd.CommandText = @"
INSERT INTO player_kv(player_id, ns, k, val, updated_at)
VALUES($pid, $ns, $k, $val, $ts)
ON CONFLICT(player_id, ns, k) DO UPDATE SET
    val = excluded.val,
    updated_at = excluded.updated_at;";
        cmd.Parameters.AddWithValue("$pid", playerId);
        cmd.Parameters.AddWithValue("$ns", ns);
        cmd.Parameters.AddWithValue("$k", key);
        cmd.Parameters.AddWithValue("$val", value);
        cmd.Parameters.AddWithValue("$ts", DateTimeOffset.UtcNow.ToUnixTimeSeconds());
        cmd.ExecuteNonQuery();
    }

    public static HashSet<int> GetFinishedBpTasks(int playerId)
    {
        var raw = playerId > 0 ? GetKv(playerId, "bp", "finished_tasks") : null;
        if (string.IsNullOrEmpty(raw)) return new HashSet<int>();
        try
        {
            return new HashSet<int>(JsonSerializer.Deserialize<List<int>>(raw) ?? new());
        }
        catch
        {
            return new HashSet<int>();
        }
    }

    public static void MarkBpTasksFinished(int playerId, IEnumerable<int> taskIds)
    {
        if (playerId <= 0) return;
        var set = GetFinishedBpTasks(playerId);
        bool changed = false;
        foreach (var tid in taskIds)
        {
            if (tid > 0 && set.Add(tid)) changed = true;
        }
        if (changed) SetKv(playerId, "bp", "finished_tasks", JsonSerializer.Serialize(set));
    }

    private static readonly Regex ZonIntPairRegex = new(
        @"\.key\s*=\s*""([^""]+)""\s*,\s*\.val\s*=\s*(-?\d+)",
        RegexOptions.Compiled);

    private static readonly Regex ZonClaimsBlockRegex = new(
        @"\.claims\s*=\s*\.\{(.*?)\}",
        RegexOptions.Compiled | RegexOptions.Singleline);

    private static readonly Regex ZonQuotedStrRegex = new(
        @"""([^""]+)""",
        RegexOptions.Compiled);

    public static (Dictionary<string, long> Ints, HashSet<string> Claims) GetProgressState(int playerId)
    {
        var ints = new Dictionary<string, long>(StringComparer.Ordinal);
        var claims = new HashSet<string>(StringComparer.Ordinal);
        if (playerId <= 0) return (ints, claims);

        string? content = null;
        using (var conn = OpenConnection())
        using (var cmd = conn.CreateCommand())
        {
            cmd.CommandText = "SELECT content FROM state_files WHERE path = $p;";
            cmd.Parameters.AddWithValue("$p", $"player/{playerId}/progress_info");
            content = cmd.ExecuteScalar() as string;
        }
        if (string.IsNullOrEmpty(content)) return (ints, claims);

        foreach (Match m in ZonIntPairRegex.Matches(content))
        {
            if (long.TryParse(m.Groups[2].Value, out long v))
            {
                ints[m.Groups[1].Value] = v;
            }
        }

        var cm = ZonClaimsBlockRegex.Match(content);
        if (cm.Success)
        {
            foreach (Match sm in ZonQuotedStrRegex.Matches(cm.Groups[1].Value))
            {
                claims.Add(sm.Groups[1].Value);
            }
        }
        return (ints, claims);
    }

    public static (long PlayerCount, long FileCount) GetDatabaseStats()
    {
        using var conn = OpenConnection();
        long pCount = 0;
        long fCount = 0;
        using (var c1 = conn.CreateCommand())
        {
            c1.CommandText = "SELECT COUNT(*) FROM players;";
            pCount = (long)(c1.ExecuteScalar() ?? 0L);
        }
        using (var c2 = conn.CreateCommand())
        {
            c2.CommandText = "SELECT COUNT(*) FROM state_files;";
            fCount = (long)(c2.ExecuteScalar() ?? 0L);
        }
        return (pCount, fCount);
    }
}
