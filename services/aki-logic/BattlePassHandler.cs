using System;
using System.Collections.Generic;
using System.Linq;
using Aki.Protocol;
using Google.Protobuf;

namespace AkiLogicService;

public static class BattlePassHandler
{
    private const long FarFuture = 4070908800; // Year 2099

    public static AkiResult HandleBattlePassRequest(int playerId, byte[] payload)
    {
        var row = SqliteStore.GetOrCreateBattlePass(playerId);
        int bpId = Math.Max(row.BpId, BinDataCache.BattlePassMeta.LatestBpId);
        var resp = new BattlePassResponse
        {
            ErrorCode = ErrorCode.Success,
            BattlePass = new PbBattlePass
            {
                InTimeRange = true,
                Id = bpId,
                Level = row.Level,
                Exp = row.Exp,
                WeeklyTotalExp = row.WeeklyExp,
                PayStatus = (BattlePassPayStatus)row.PayStatus,
                BeginTime = 0,
                EndTime = FarFuture,
                HadEnter = row.HadEnter
            }
        };

        bool repaired = false;
        foreach (var entry in row.TakenRewards.ToList())
        {
            if (entry.Length < 2) continue;
            int level = entry[0];
            int typeInt = entry[1];
            int itemId = entry.Length >= 3 ? entry[2] : 0;
            if (itemId > 0)
            {
                resp.BattlePass.TakenRewards.Add(new PbBattlePassReward
                {
                    Level = level,
                    Type = (BattlePassType)typeInt,
                    ItemId = itemId
                });
                continue;
            }

            // Heal legacy SQLite rows that stored ItemId=0 so the client never logs missing reward config.
            var resolved = ResolveRewardItemIds(bpId, level, typeInt);
            if (resolved.Count == 0) continue;
            row.TakenRewards.Remove(entry);
            foreach (int resolvedId in resolved)
            {
                row.TakenRewards.Add(new[] { level, typeInt, resolvedId });
                resp.BattlePass.TakenRewards.Add(new PbBattlePassReward
                {
                    Level = level,
                    Type = (BattlePassType)typeInt,
                    ItemId = resolvedId
                });
            }
            repaired = true;
        }
        if (repaired)
        {
            SqliteStore.SaveBattlePass(row);
        }

        Console.WriteLine($"[BattlePassHandler] Served player {playerId} BP Lv.{row.Level} (taken={row.TakenRewards.Count}) from SQLite.");
        return AkiResult.Ok(EMessageId.BattlePassResponse, resp);
    }

    public static AkiResult HandleBattlePassTaskRequest(int playerId, byte[] payload)
    {
        var row = SqliteStore.GetOrCreateBattlePass(playerId);
        var takenSet = row.TakenTasks.ToHashSet();
        var finishedSet = SqliteStore.GetFinishedBpTasks(playerId);

        var resp = new BattlePassTaskResponse
        {
            ErrorCode = ErrorCode.Success,
            DayEnd = FarFuture,
            WeekEnd = FarFuture
        };

        foreach (var task in BinDataCache.BattlePassTasks.Values)
        {
            bool isTaken = takenSet.Contains(task.TaskId);
            bool isFinished = isTaken || finishedSet.Contains(task.TaskId) || task.UpdateType == 1;
            resp.Tasks.Add(new PbBattlePassTask
            {
                Id = task.TaskId,
                Current = isFinished ? 1 : 0,
                Target = 1,
                IsFinished = isFinished,
                IsTaken = isTaken
            });
        }

        return AkiResult.Ok(EMessageId.BattlePassTaskResponse, resp);
    }

    private static BinDataCache.BpRewardEntry? ResolveRewardEntry(int bpId, int level)
    {
        var candidateBpIds = new[] { BinDataCache.BattlePassMeta.LatestRewardId, bpId, 0, 1 };
        foreach (var cand in candidateBpIds)
        {
            if (BinDataCache.BattlePassRewards.TryGetValue((cand, level), out var cfg))
            {
                return cfg;
            }
        }
        return null;
    }

    private static List<int> ResolveRewardItemIds(int bpId, int level, int typeInt)
    {
        var cfg = ResolveRewardEntry(bpId, level);
        if (cfg == null) return new List<int>();
        var map = typeInt == 0 ? cfg.FreeReward : cfg.PayReward;
        return map.Keys.Where(id => id > 0).ToList();
    }

    private static void CollectLevelGrants(int bpId, int level, int typeInt, Dictionary<int, int> acc)
    {
        var cfg = ResolveRewardEntry(bpId, level);
        if (cfg == null) return;
        var map = typeInt == 0 ? cfg.FreeReward : cfg.PayReward;
        foreach (var kv in map)
        {
            acc[kv.Key] = acc.GetValueOrDefault(kv.Key, 0) + kv.Value;
        }
    }

    public static AkiResult HandleBattlePassTakeRewardRequest(int playerId, byte[] payload)
    {
        var req = new BattlePassTakeRewardRequest();
        if (payload.Length > 0) req.MergeFrom(payload);

        var row = SqliteStore.GetOrCreateBattlePass(playerId);
        int bpId = Math.Max(row.BpId, BinDataCache.BattlePassMeta.LatestBpId);
        int typeInt = (int)req.Type;
        var grants = new Dictionary<int, int>();

        bool alreadyHasLevelType = row.TakenRewards.Any(x => x.Length >= 2 && x[0] == req.Level && x[1] == typeInt);
        bool alreadyHasItem = req.ItemId > 0
            ? row.TakenRewards.Any(x => x.Length >= 3 && x[0] == req.Level && x[1] == typeInt && (x[2] == req.ItemId || x[2] == 0))
            : alreadyHasLevelType;

        if (req.Level > 0 && req.Level <= row.Level && !alreadyHasItem)
        {
            if (req.ItemId > 0)
            {
                row.TakenRewards.Add(new[] { req.Level, typeInt, req.ItemId });
            }
            else
            {
                foreach (int resolvedId in ResolveRewardItemIds(bpId, req.Level, typeInt))
                {
                    row.TakenRewards.Add(new[] { req.Level, typeInt, resolvedId });
                }
            }
            SqliteStore.SaveBattlePass(row);
            if (!alreadyHasLevelType)
            {
                CollectLevelGrants(row.BpId, req.Level, typeInt, grants);
            }
        }

        var result = AkiResult.Ok(EMessageId.BattlePassTakeRewardResponse, new BattlePassTakeRewardResponse { ErrorCode = ErrorCode.Success });
        foreach (var kv in grants) result.WithGrant(kv.Key, kv.Value);
        return result;
    }

    public static AkiResult HandleBattlePassTakeAllRewardRequest(int playerId, byte[] payload)
    {
        var row = SqliteStore.GetOrCreateBattlePass(playerId);
        int bpId = Math.Max(row.BpId, BinDataCache.BattlePassMeta.LatestBpId);
        var resp = new BattlePassTakeAllRewardResponse
        {
            ErrorCode = ErrorCode.Success
        };
        var grants = new Dictionary<int, int>();
        int maxType = row.PayStatus > 0 ? 1 : 0;

        for (int l = 1; l <= row.Level; l++)
        {
            for (int t = 0; t <= maxType; t++)
            {
                if (!row.TakenRewards.Any(x => x.Length >= 2 && x[0] == l && x[1] == t))
                {
                    var itemIds = ResolveRewardItemIds(bpId, l, t);
                    foreach (int itemId in itemIds)
                    {
                        row.TakenRewards.Add(new[] { l, t, itemId });
                        resp.TakenRewards.Add(new PbBattlePassReward
                        {
                            Level = l,
                            Type = (BattlePassType)t,
                            ItemId = itemId
                        });
                    }
                    CollectLevelGrants(row.BpId, l, t, grants);
                }
            }
        }

        SqliteStore.SaveBattlePass(row);
        var result = AkiResult.Ok(EMessageId.BattlePassTakeAllRewardResponse, resp);
        foreach (var kv in grants) result.WithGrant(kv.Key, kv.Value);
        return result;
    }

    public static AkiResult HandleBattlePassLevelUpRequest(int playerId, byte[] payload)
    {
        var req = new BattlePassLevelUpRequest();
        if (payload.Length > 0) req.MergeFrom(payload);

        var row = SqliteStore.GetOrCreateBattlePass(playerId);
        var newLevel = Math.Clamp(row.Level + Math.Max(1, req.Level), 1, 70);
        var updated = row with { Level = newLevel };
        SqliteStore.SaveBattlePass(updated);

        var notify = new BattlePassExpUpdateNotify
        {
            Level = newLevel,
            Exp = updated.Exp,
            WeeklyTotalExp = updated.WeeklyExp
        };

        return AkiResult.Ok(EMessageId.BattlePassLevelUpResponse, new BattlePassLevelUpResponse { ErrorCode = ErrorCode.Success })
            .WithPush(EMessageId.BattlePassExpUpdateNotify, notify);
    }

    public static AkiResult HandleBattlePassRecurringTakeRequest(int playerId, byte[] payload)
    {
        return AkiResult.Ok(EMessageId.BattlePassRecurringTakeResponse, new BattlePassRecurringTakeResponse { ErrorCode = ErrorCode.Success });
    }
}
