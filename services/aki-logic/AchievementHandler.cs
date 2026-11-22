using System;
using System.Collections.Generic;
using System.Text.Json;
using Aki.Protocol;
using Google.Protobuf;

namespace AkiLogicService;

public static class AchievementHandler
{
    public static AkiResult HandleAchievementInfoRequest(int playerId, byte[] payload)
    {
        var entries = SqliteStore.GetAchievements(playerId);
        var rawGroups = SqliteStore.GetKv(playerId, "achievement", "claimed_groups");
        var claimedGroups = string.IsNullOrEmpty(rawGroups)
            ? new HashSet<int>()
            : new HashSet<int>(JsonSerializer.Deserialize<List<int>>(rawGroups) ?? new());

        var achByGroup = new Dictionary<int, List<BinDataCache.AchievementCfg>>();
        var levelById = new Dictionary<int, int>();
        foreach (var ach in BinDataCache.Achievements)
        {
            levelById[ach.Id] = Math.Max(1, ach.Level);
            if (!achByGroup.TryGetValue(ach.GroupId, out var list))
            {
                list = new List<BinDataCache.AchievementCfg>();
                achByGroup[ach.GroupId] = list;
            }
            list.Add(ach);
        }

        int finishCount = 0;
        int starCount = 0;
        foreach (var kv in entries)
        {
            finishCount++;
            starCount += levelById.GetValueOrDefault(kv.Key, 1);
        }

        var resp = new AchievementInfoResponse
        {
            StarCount = starCount,
            FinishCount = finishCount
        };

        uint nowU32 = (uint)Math.Clamp(DateTimeOffset.UtcNow.ToUnixTimeSeconds(), 1, uint.MaxValue);
        foreach (var grp in BinDataCache.AchievementGroups)
        {
            if (!grp.Enable) continue;
            var grpInfo = new AchievementGroupInfo
            {
                AchievementGroupEntry = new AchievementGroupEntry
                {
                    Id = grp.Id,
                    FinishTime = 0,
                    IsReceive = claimedGroups.Contains(grp.Id)
                }
            };

            if (achByGroup.TryGetValue(grp.Id, out var list))
            {
                foreach (var cfg in list)
                {
                    if (entries.TryGetValue(cfg.Id, out var rec))
                    {
                        grpInfo.AchievementGroupEntry.FinishTime = 1;
                        grpInfo.AchievementEntryList.Add(new AchievementEntry
                        {
                            Id = cfg.Id,
                            FinishTime = nowU32,
                            IsReceive = rec.IsTaken,
                            Progress = new AchievementProgress { CurProgress = rec.Progress, TotalProgress = 1 }
                        });
                    }
                }
            }
            resp.AchievementGroupInfoList.Add(grpInfo);
        }

        return AkiResult.Ok(EMessageId.AchievementInfoResponse, resp);
    }

    public static AkiResult HandleAchievementUpdateRequest(int playerId, byte[] payload)
    {
        var entries = SqliteStore.GetAchievements(playerId);
        var resp = new AchievementUpdateResponse { ErrorCode = ErrorCode.Success };
        uint nowU32 = (uint)Math.Clamp(DateTimeOffset.UtcNow.ToUnixTimeSeconds(), 1, uint.MaxValue);

        foreach (var kv in entries)
        {
            resp.AchievementEntryList.Add(new AchievementEntry
            {
                Id = kv.Key,
                FinishTime = nowU32,
                IsReceive = kv.Value.IsTaken,
                Progress = new AchievementProgress { CurProgress = kv.Value.Progress, TotalProgress = 1 }
            });
        }

        return AkiResult.Ok(EMessageId.AchievementUpdateResponse, resp);
    }

    public static AkiResult HandleAchievementFinishRequest(int playerId, byte[] payload)
    {
        var req = new AchievementFinishRequest();
        if (payload.Length > 0) req.MergeFrom(payload);

        if (req.Id <= 0)
        {
            return AkiResult.Ok(EMessageId.AchievementFinishResponse, new AchievementFinishResponse
            {
                ErrorCode = ErrorCode.RequestParamError
            });
        }

        SqliteStore.FinishAchievement(playerId, req.Id, 1);
        uint nowU32 = (uint)Math.Clamp(DateTimeOffset.UtcNow.ToUnixTimeSeconds(), 1, uint.MaxValue);

        var entries = SqliteStore.GetAchievements(playerId);
        var levelById = new Dictionary<int, int>();
        foreach (var ach in BinDataCache.Achievements)
        {
            levelById[ach.Id] = Math.Max(1, ach.Level);
        }
        int finishCount = entries.Count;
        int starCount = 0;
        foreach (var kv in entries)
        {
            starCount += levelById.GetValueOrDefault(kv.Key, 1);
        }

        var notify = new AchievementProgressNotify
        {
            AchievementEntry = new AchievementEntry
            {
                Id = req.Id,
                FinishTime = nowU32,
                IsReceive = false,
                Progress = new AchievementProgress { CurProgress = 1, TotalProgress = 1 }
            }
        };

        var countNotify = new AchievementCountChangeNotify
        {
            StarCount = starCount,
            FinishCount = finishCount
        };

        return AkiResult.Ok(EMessageId.AchievementFinishResponse, new AchievementFinishResponse { ErrorCode = ErrorCode.Success })
            .WithPush(EMessageId.AchievementProgressNotify, notify)
            .WithPush(EMessageId.AchievementCountChangeNotify, countNotify);
    }

    private static bool TryClaimSingle(
        int playerId,
        int id,
        bool isGroupId,
        Dictionary<int, (int Progress, bool IsTaken)> entries,
        HashSet<int> claimedGroups,
        Dictionary<int, int> outGrants,
        List<int> newlyTakenIds)
    {
        if (isGroupId)
        {
            var grp = BinDataCache.AchievementGroups.FirstOrDefault(g => g.Id == id);
            if (grp == null || claimedGroups.Contains(id)) return false;
            claimedGroups.Add(id);
            var drops = BinDataCache.ResolveDrop(grp.DropId);
            if (drops.Count == 0) drops.Add((3, 20));
            foreach (var (iid, cnt) in drops) outGrants[iid] = outGrants.GetValueOrDefault(iid, 0) + cnt;
            return true;
        }
        else
        {
            var cfg = BinDataCache.Achievements.FirstOrDefault(a => a.Id == id);
            if (cfg == null) return false;
            if (!entries.TryGetValue(id, out var rec) || rec.IsTaken) return false;

            newlyTakenIds.Add(id);
            var drops = BinDataCache.ResolveDrop(cfg.OverrideDropId);
            if (drops.Count == 0)
            {
                int astrite = Math.Clamp(cfg.Level, 1, 3) switch { 1 => 5, 2 => 10, _ => 20 };
                drops.Add((3, astrite));
            }
            foreach (var (iid, cnt) in drops) outGrants[iid] = outGrants.GetValueOrDefault(iid, 0) + cnt;
            return true;
        }
    }

    public static AkiResult HandleAchievementReceiveRequest(int playerId, byte[] payload)
    {
        var req = new AchievementReceiveRequest();
        if (payload.Length > 0) req.MergeFrom(payload);

        var entries = SqliteStore.GetAchievements(playerId);
        var rawGroups = SqliteStore.GetKv(playerId, "achievement", "claimed_groups");
        var claimedGroups = string.IsNullOrEmpty(rawGroups)
            ? new HashSet<int>()
            : new HashSet<int>(JsonSerializer.Deserialize<List<int>>(rawGroups) ?? new());

        var grants = new Dictionary<int, int>();
        var takenIds = new List<int>();
        bool ok = TryClaimSingle(playerId, req.Id, req.IsGroupId, entries, claimedGroups, grants, takenIds);
        if (!ok)
        {
            return AkiResult.Ok(EMessageId.AchievementReceiveResponse, new AchievementReceiveResponse { ErrorCode = ErrorCode.RequestParamError });
        }

        if (takenIds.Count > 0) SqliteStore.MarkAchievementsTaken(playerId, takenIds);
        if (req.IsGroupId) SqliteStore.SetKv(playerId, "achievement", "claimed_groups", JsonSerializer.Serialize(claimedGroups));

        var resp = new AchievementReceiveResponse { ErrorCode = ErrorCode.Success };
        foreach (var kv in grants) resp.ItemMap[kv.Key] = kv.Value;

        var result = AkiResult.Ok(EMessageId.AchievementReceiveResponse, resp);
        foreach (var kv in grants) result.WithGrant(kv.Key, kv.Value);
        return result;
    }

    public static AkiResult HandleMulAchievementReceiveRequest(int playerId, byte[] payload)
    {
        var req = new MulAchievementReceiveRequest();
        if (payload.Length > 0) req.MergeFrom(payload);

        var entries = SqliteStore.GetAchievements(playerId);
        var rawGroups = SqliteStore.GetKv(playerId, "achievement", "claimed_groups");
        var claimedGroups = string.IsNullOrEmpty(rawGroups)
            ? new HashSet<int>()
            : new HashSet<int>(JsonSerializer.Deserialize<List<int>>(rawGroups) ?? new());

        var grants = new Dictionary<int, int>();
        var takenIds = new List<int>();
        var resp = new MulAchievementReceiveResponse { ErrorCode = ErrorCode.Success };
        bool groupsChanged = false;

        foreach (var aid in req.AchievementIds)
        {
            if (TryClaimSingle(playerId, aid, false, entries, claimedGroups, grants, takenIds))
            {
                resp.AchievementIds.Add(aid);
            }
        }
        foreach (var gid in req.AchievementGroupIds)
        {
            if (TryClaimSingle(playerId, gid, true, entries, claimedGroups, grants, takenIds))
            {
                resp.AchievementGroupIds.Add(gid);
                groupsChanged = true;
            }
        }

        if (takenIds.Count > 0) SqliteStore.MarkAchievementsTaken(playerId, takenIds);
        if (groupsChanged) SqliteStore.SetKv(playerId, "achievement", "claimed_groups", JsonSerializer.Serialize(claimedGroups));

        var result = AkiResult.Ok(EMessageId.MulAchievementReceiveResponse, resp);
        foreach (var kv in grants) result.WithGrant(kv.Key, kv.Value);
        return result;
    }
}
