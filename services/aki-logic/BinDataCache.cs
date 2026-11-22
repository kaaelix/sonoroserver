using System;
using System.Collections.Generic;
using System.IO;
using System.Text.Json;

namespace AkiLogicService;

public static class BinDataCache
{
    public static readonly string RepoRoot = ResolveRepoRoot();

    public static readonly string BinDataDir = Environment.GetEnvironmentVariable("AKI_BINDATA_DIR")
        ?? Path.Combine(RepoRoot, "assets", "BinData");

    private static string ResolveRepoRoot()
    {
        foreach (var start in new[] { AppContext.BaseDirectory, Directory.GetCurrentDirectory() })
        {
            var dir = new DirectoryInfo(start);
            while (dir != null)
            {
                if (Directory.Exists(Path.Combine(dir.FullName, "assets", "BinData")) ||
                    File.Exists(Path.Combine(dir.FullName, "build.zig")))
                {
                    return dir.FullName;
                }
                dir = dir.Parent;
            }
        }
        return Directory.GetCurrentDirectory();
    }

    public record BpRewardEntry(int BattlePassId, int Level, Dictionary<int, int> FreeReward, Dictionary<int, int> PayReward);
    public record BpTaskCfg(int TaskId, int UpdateType, Dictionary<int, int> TaskReward);
    public record BpMeta(int LatestBpId, int LatestRewardId, int WeekExpLimit);
    public record LivenessBox(int Id, int Goal, int DropId);
    public record LivenessTaskEntry(int TaskId, Dictionary<int, int> TaskReward);
    public record AchievementCfg(int Id, int GroupId, int Level, int OverrideDropId);
    public record AchievementGroupCfg(int Id, bool Enable, int DropId);
    public record PayItemDef(int Id, int PayId, int ItemId, int ItemCount, int BonusItemCount, int SpecialBonusItemCount, string StageImage, string ProductId, string Amount);

    private static readonly Lazy<Dictionary<int, List<(int ItemId, int Count)>>> _dropPackages = new(LoadDropPackages);
    private static readonly Lazy<Dictionary<(int BpId, int Level), BpRewardEntry>> _bpRewards = new(LoadBpRewards);
    private static readonly Lazy<BpMeta> _bpMeta = new(LoadBpMeta);
    private static readonly Lazy<Dictionary<int, BpTaskCfg>> _bpTasks = new(LoadBpTasks);
    private static readonly Lazy<List<int>> _bpTaskIds = new(() => new List<int>(_bpTasks.Value.Keys));
    private static readonly Lazy<Dictionary<int, int>> _adventureTaskDrops = new(LoadAdventureTaskDrops);
    private static readonly Lazy<Dictionary<int, int>> _payGiftItems = new(LoadPayGiftItems);
    private static readonly Lazy<List<PayItemDef>> _payItems = new(LoadPayItems);
    private static readonly Lazy<List<int>> _accessPathIds = new(LoadAccessPathIds);
    private static readonly Lazy<List<LivenessBox>> _livenessBoxes = new(LoadLivenessBoxes);
    private static readonly Lazy<List<LivenessTaskEntry>> _livenessTasks = new(LoadLivenessTasks);
    private static readonly Lazy<List<AchievementCfg>> _achievements = new(LoadAchievements);
    private static readonly Lazy<List<AchievementGroupCfg>> _achievementGroups = new(LoadAchievementGroups);
    private static readonly Lazy<HashSet<int>> _guideGroupIds = new(LoadGuideGroupIds);
    private static readonly Lazy<List<int>> _playerTitleIds = new(LoadPlayerTitleIds);
    private static readonly Lazy<List<int>> _monsterDetectionIds = new(LoadMonsterDetectionIds);
    private static readonly Lazy<List<int>> _dungeonDetectionIds = new(() => LoadIdList("DungeonDetection.json"));
    private static readonly Lazy<List<int>> _silentAreaDetectionIds = new(() => LoadIdList("SilentAreaDetection.json"));
    private static readonly Lazy<(Dictionary<int, List<int>> Albums, HashSet<int> AllMusicIds)> _musicData = new(LoadMusicData);

    public static IReadOnlyDictionary<int, List<(int ItemId, int Count)>> DropPackages => _dropPackages.Value;
    public static IReadOnlyDictionary<(int BpId, int Level), BpRewardEntry> BattlePassRewards => _bpRewards.Value;
    public static BpMeta BattlePassMeta => _bpMeta.Value;
    public static IReadOnlyDictionary<int, BpTaskCfg> BattlePassTasks => _bpTasks.Value;
    public static IReadOnlyList<int> BattlePassTaskIds => _bpTaskIds.Value;
    public static IReadOnlyDictionary<int, int> AdventureTaskDrops => _adventureTaskDrops.Value;
    public static IReadOnlyDictionary<int, int> PayGiftItems => _payGiftItems.Value;
    public static IReadOnlyList<PayItemDef> PayItems => _payItems.Value;
    public static IReadOnlyList<int> AccessPathIds => _accessPathIds.Value;
    public static IReadOnlyList<LivenessBox> LivenessBoxes => _livenessBoxes.Value;
    public static IReadOnlyList<LivenessTaskEntry> LivenessTasks => _livenessTasks.Value;
    public static IReadOnlyList<AchievementCfg> Achievements => _achievements.Value;
    public static IReadOnlyList<AchievementGroupCfg> AchievementGroups => _achievementGroups.Value;
    public static IReadOnlySet<int> GuideGroupIds => _guideGroupIds.Value;
    public static IReadOnlyList<int> PlayerTitleIds => _playerTitleIds.Value;
    public static IReadOnlyList<int> MonsterDetectionIds => _monsterDetectionIds.Value;
    public static IReadOnlyList<int> DungeonDetectionIds => _dungeonDetectionIds.Value;
    public static IReadOnlyList<int> SilentAreaDetectionIds => _silentAreaDetectionIds.Value;
    public static IReadOnlyDictionary<int, List<int>> PhonographAlbums => _musicData.Value.Albums;
    public static IReadOnlySet<int> AllMusicIds => _musicData.Value.AllMusicIds;

    public static List<(int ItemId, int Count)> ResolveDrop(int dropId)
    {
        if (dropId > 0 && DropPackages.TryGetValue(dropId, out var items) && items.Count > 0)
        {
            return new List<(int, int)>(items);
        }
        return new List<(int, int)>();
    }

    private static Dictionary<int, int> ReadIntMap(JsonElement obj, string propName)
    {
        var map = new Dictionary<int, int>();
        if (!obj.TryGetProperty(propName, out var el) || el.ValueKind != JsonValueKind.Object) return map;
        foreach (var kv in el.EnumerateObject())
        {
            if (int.TryParse(kv.Name, out int k) && kv.Value.TryGetInt32(out int v) && k > 0 && v > 0)
            {
                map[k] = v;
            }
        }
        return map;
    }

    private static JsonDocument? OpenJson(string fileName)
    {
        var path = Path.Combine(BinDataDir, fileName);
        if (!File.Exists(path)) return null;
        using var fs = File.OpenRead(path);
        return JsonDocument.Parse(fs);
    }

    private static Dictionary<int, List<(int, int)>> LoadDropPackages()
    {
        var res = new Dictionary<int, List<(int, int)>>();
        using var doc = OpenJson("DropPackage.json");
        if (doc == null) return res;
        foreach (var el in doc.RootElement.EnumerateArray())
        {
            if (!el.TryGetProperty("Id", out var idEl) || !idEl.TryGetInt32(out int id)) continue;
            var map = ReadIntMap(el, "DropPreview");
            var list = new List<(int, int)>();
            foreach (var kv in map) list.Add((kv.Key, kv.Value));
            res[id] = list;
        }
        return res;
    }

    private static Dictionary<(int BpId, int Level), BpRewardEntry> LoadBpRewards()
    {
        var res = new Dictionary<(int BpId, int Level), BpRewardEntry>();
        using var doc = OpenJson("BattlePassReward.json");
        if (doc == null) return res;
        foreach (var el in doc.RootElement.EnumerateArray())
        {
            int bpId = el.TryGetProperty("BattlePassId", out var b) ? b.GetInt32() : 0;
            int level = el.TryGetProperty("Level", out var l) ? l.GetInt32() : 0;
            if (level <= 0) continue;
            var key = (bpId, level);
            if (res.ContainsKey(key)) continue;
            res[key] = new BpRewardEntry(bpId, level, ReadIntMap(el, "FreeReward"), ReadIntMap(el, "PayReward"));
        }
        return res;
    }

    private static BpMeta LoadBpMeta()
    {
        int latestId = 2;
        int latestRewardId = 13;
        int weekExpLimit = 12000;
        using var doc = OpenJson("BattlePass.json");
        if (doc == null) return new BpMeta(latestId, latestRewardId, weekExpLimit);
        foreach (var el in doc.RootElement.EnumerateArray())
        {
            if (el.TryGetProperty("Id", out var idEl) && idEl.TryGetInt32(out int id) && id > 0)
            {
                latestId = id;
                if (el.TryGetProperty("BattlePassRewardId", out var rwEl) && rwEl.TryGetInt32(out int rwid))
                {
                    latestRewardId = rwid;
                }
                if (el.TryGetProperty("WeekExpLimit", out var wlEl) && wlEl.TryGetInt32(out int wl) && wl > 0)
                {
                    weekExpLimit = wl;
                }
            }
        }
        return new BpMeta(latestId, latestRewardId, weekExpLimit);
    }

    private static Dictionary<int, BpTaskCfg> LoadBpTasks()
    {
        var res = new Dictionary<int, BpTaskCfg>();
        using var doc = OpenJson("BattlePassTask.json");
        if (doc == null) return res;
        foreach (var el in doc.RootElement.EnumerateArray())
        {
            if (!el.TryGetProperty("TaskId", out var idEl) || !idEl.TryGetInt32(out int tid) || tid <= 0) continue;
            int updateType = el.TryGetProperty("UpdateType", out var utEl) && utEl.TryGetInt32(out int ut) ? ut : 0;
            res[tid] = new BpTaskCfg(tid, updateType, ReadIntMap(el, "TaskReward"));
        }
        return res;
    }

    private static Dictionary<int, int> LoadAdventureTaskDrops()
    {
        var res = new Dictionary<int, int>();
        using var doc = OpenJson("AdventureTask.json");
        if (doc == null) return res;
        foreach (var el in doc.RootElement.EnumerateArray())
        {
            if (el.TryGetProperty("Id", out var idEl) && idEl.TryGetInt32(out int id) && id > 0)
            {
                int dropId = el.TryGetProperty("DropIds", out var dEl) && dEl.TryGetInt32(out int d) ? d : 0;
                res[id] = dropId;
            }
        }
        return res;
    }

    private static Dictionary<int, int> LoadPayGiftItems()
    {
        var res = new Dictionary<int, int>();
        using var doc = OpenJson("PayGift.json");
        if (doc == null) return res;
        foreach (var el in doc.RootElement.EnumerateArray())
        {
            if (el.TryGetProperty("Id", out var idEl) && idEl.TryGetInt32(out int id) && id > 0)
            {
                int itemId = el.TryGetProperty("ItemId", out var itEl) && itEl.TryGetInt32(out int it) ? it : 0;
                if (itemId > 0) res[id] = itemId;
            }
        }
        return res;
    }

    private static List<PayItemDef> LoadPayItems()
    {
        var payMeta = new Dictionary<int, (string ProductId, string Amount)>();
        using (var payDoc = OpenJson("Pay.json"))
        {
            if (payDoc != null)
            {
                foreach (var el in payDoc.RootElement.EnumerateArray())
                {
                    if (!el.TryGetProperty("PayId", out var pidEl) || !pidEl.TryGetInt32(out int payId)) continue;
                    string region = el.TryGetProperty("Region", out var rEl) ? (rEl.GetString() ?? "") : "";
                    if (region != "Default" && payMeta.ContainsKey(payId)) continue;
                    string prod = el.TryGetProperty("ProductId", out var prEl) ? (prEl.GetString() ?? $"com.kurogame.aki.pay{payId}") : $"com.kurogame.aki.pay{payId}";
                    double amt = el.TryGetProperty("Amount", out var aEl) && aEl.TryGetDouble(out double a) ? a : 0.99;
                    payMeta[payId] = (prod, $"${amt:0.##}");
                }
            }
        }

        var res = new List<PayItemDef>();
        using var doc = OpenJson("PayItem.json");
        if (doc == null) return res;
        foreach (var el in doc.RootElement.EnumerateArray())
        {
            if (!el.TryGetProperty("Id", out var idEl) || !idEl.TryGetInt32(out int id) || id <= 0 || id > 6) continue;
            int payId = el.TryGetProperty("PayId", out var pEl) && pEl.TryGetInt32(out int p) ? p : id;
            int itemId = el.TryGetProperty("ItemId", out var itEl) && itEl.TryGetInt32(out int it) ? it : 4;
            int itemCount = el.TryGetProperty("ItemCount", out var icEl) && icEl.TryGetInt32(out int ic) ? ic : 60;
            int bonus = el.TryGetProperty("BonusItemCount", out var bEl) && bEl.TryGetInt32(out int b) ? b : 0;
            int special = el.TryGetProperty("SpecialBonusItemCount", out var sEl) && sEl.TryGetInt32(out int s) ? s : 0;
            string stageImg = el.TryGetProperty("StageImage", out var imgEl) ? (imgEl.GetString() ?? "") : "";
            var (prod, amt) = payMeta.TryGetValue(payId, out var m) ? m : ($"com.kurogame.aki.pay{payId}", "$0.99");
            res.Add(new PayItemDef(id, payId, itemId, itemCount, bonus, special, stageImg, prod, amt));
        }
        return res;
    }

    private static List<int> LoadAccessPathIds()
    {
        var res = new List<int>();
        using var doc = OpenJson("AccessPath.json");
        if (doc == null) return res;
        foreach (var el in doc.RootElement.EnumerateArray())
        {
            if (el.TryGetProperty("Id", out var idEl) && idEl.TryGetInt32(out int id) && id > 0)
            {
                res.Add(id);
                if (res.Count >= 128) break;
            }
        }
        return res;
    }

    private static List<LivenessBox> LoadLivenessBoxes()
    {
        var res = new List<LivenessBox>();
        using var doc = OpenJson("Liveness.json");
        if (doc == null) return res;
        foreach (var el in doc.RootElement.EnumerateArray())
        {
            int id = el.TryGetProperty("Id", out var p1) ? p1.GetInt32() : 0;
            int goal = el.TryGetProperty("Goal", out var p2) ? p2.GetInt32() : 0;
            int dropId = el.TryGetProperty("DropId", out var p3) ? p3.GetInt32() : 0;
            if (id > 0) res.Add(new LivenessBox(id, goal, dropId));
        }
        return res;
    }

    private static List<LivenessTaskEntry> LoadLivenessTasks()
    {
        var res = new List<LivenessTaskEntry>();
        using var doc = OpenJson("LivenessTask.json");
        if (doc == null) return res;
        foreach (var el in doc.RootElement.EnumerateArray())
        {
            if (el.TryGetProperty("TaskId", out var p1) && p1.TryGetInt32(out int tid))
            {
                res.Add(new LivenessTaskEntry(tid, ReadIntMap(el, "TaskReward")));
            }
        }
        return res;
    }

    private static List<AchievementCfg> LoadAchievements()
    {
        var res = new List<AchievementCfg>();
        using var doc = OpenJson("Achievement.json");
        if (doc == null) return res;
        foreach (var el in doc.RootElement.EnumerateArray())
        {
            int id = el.TryGetProperty("Id", out var p1) ? p1.GetInt32() : 0;
            int gid = el.TryGetProperty("GroupId", out var p2) ? p2.GetInt32() : 0;
            int lvl = el.TryGetProperty("Level", out var p3) ? p3.GetInt32() : 1;
            int drop = el.TryGetProperty("OverrideDropId", out var p4) ? p4.GetInt32() : 0;
            if (id > 0) res.Add(new AchievementCfg(id, gid, lvl, drop));
        }
        return res;
    }

    private static List<AchievementGroupCfg> LoadAchievementGroups()
    {
        var res = new List<AchievementGroupCfg>();
        using var doc = OpenJson("AchievementGroup.json");
        if (doc == null) return res;
        foreach (var el in doc.RootElement.EnumerateArray())
        {
            int id = el.TryGetProperty("Id", out var p1) ? p1.GetInt32() : 0;
            bool enable = el.TryGetProperty("Enable", out var p2) && p2.GetBoolean();
            int drop = el.TryGetProperty("DropId", out var p3) ? p3.GetInt32() : 0;
            if (id > 0) res.Add(new AchievementGroupCfg(id, enable, drop));
        }
        return res;
    }

    private static HashSet<int> LoadGuideGroupIds()
    {
        var res = new HashSet<int>();
        using var doc = OpenJson("GuideGroup.json");
        if (doc == null) return res;
        foreach (var el in doc.RootElement.EnumerateArray())
        {
            if (el.TryGetProperty("Id", out var p1) && p1.TryGetInt32(out int id) && id > 0)
            {
                res.Add(id);
            }
        }
        return res;
    }

    private static List<int> LoadPlayerTitleIds()
    {
        var res = new List<int>();
        using var doc = OpenJson("PlayerTitle.json");
        if (doc == null) return res;
        foreach (var el in doc.RootElement.EnumerateArray())
        {
            if (el.TryGetProperty("Id", out var p1) && p1.TryGetInt32(out int id) && id > 0)
            {
                res.Add(id);
            }
        }
        return res;
    }

    private static List<int> LoadMonsterDetectionIds() => LoadIdList("MonsterDetection.json");

    private static List<int> LoadIdList(string fileName)
    {
        var res = new List<int>();
        using var doc = OpenJson(fileName);
        if (doc == null) return res;
        foreach (var el in doc.RootElement.EnumerateArray())
        {
            if (el.TryGetProperty("Id", out var p1) && p1.TryGetInt32(out int id) && id > 0)
            {
                res.Add(id);
            }
        }
        return res;
    }

    private static (Dictionary<int, List<int>>, HashSet<int>) LoadMusicData()
    {
        var albums = new Dictionary<int, List<int>>();
        var allMusic = new HashSet<int>();

        using (var doc = OpenJson("PhonographAlbum.json"))
        {
            if (doc != null)
            {
                foreach (var el in doc.RootElement.EnumerateArray())
                {
                    if (el.TryGetProperty("Id", out var p1) && p1.TryGetInt32(out int aid) && aid > 0)
                    {
                        if (!albums.ContainsKey(aid)) albums[aid] = new List<int>();
                    }
                }
            }
        }

        using (var doc = OpenJson("PhonographMusic.json"))
        {
            if (doc != null)
            {
                foreach (var el in doc.RootElement.EnumerateArray())
                {
                    if (!el.TryGetProperty("Id", out var idEl) || !idEl.TryGetInt32(out int mid) || mid <= 0) continue;
                    allMusic.Add(mid);
                    if (el.TryGetProperty("Album", out var albEl) && albEl.ValueKind == JsonValueKind.Array)
                    {
                        foreach (var a in albEl.EnumerateArray())
                        {
                            if (a.TryGetInt32(out int aid))
                            {
                                if (!albums.TryGetValue(aid, out var list))
                                {
                                    list = new List<int>();
                                    albums[aid] = list;
                                }
                                list.Add(mid);
                            }
                        }
                    }
                }
            }
        }

        return (albums, allMusic);
    }
}
