using System;
using System.Collections.Generic;
using Aki.Protocol;
using Google.Protobuf;

namespace AkiLogicService;

public static class LivenessHandler
{
    public static AkiResult HandleLivenessRequest(int playerId, byte[] payload)
    {
        var row = SqliteStore.GetOrCreateLiveness(playerId);
        long nowSec = DateTimeOffset.UtcNow.ToUnixTimeSeconds();
        var takenSet = row.ClaimedTasks.ToHashSet();

        var info = new LivenessInfo
        {
            LivenessCount = row.LivenessCount,
            DayEnd = nowSec + 86_400,
            AreaId = 1
        };
        foreach (var boxId in row.ClaimedBoxes) info.RewardedLiveness.Add(boxId);

        foreach (var task in BinDataCache.LivenessTasks)
        {
            info.Tasks.Add(new LivenessTask
            {
                Id = task.TaskId,
                Current = 1,
                Target = 1,
                IsFinished = true,
                IsTaken = takenSet.Contains(task.TaskId),
                IsConditionUnlock = true
            });
        }

        return AkiResult.Ok(EMessageId.LivenessResponse, new LivenessResponse { LivenessInfo = info });
    }

    public static AkiResult HandleLivenessTaskTakeRequest(int playerId, byte[] payload)
    {
        var req = new LivenessTaskTakeRequest();
        if (payload.Length > 0) req.MergeFrom(payload);

        var row = SqliteStore.GetOrCreateLiveness(playerId);
        var taken = new HashSet<int>(row.ClaimedTasks);
        var taskLookup = new Dictionary<int, BinDataCache.LivenessTaskEntry>();
        foreach (var t in BinDataCache.LivenessTasks) taskLookup[t.TaskId] = t;

        var resp = new LivenessTaskTakeResponse { ErrorCode = ErrorCode.Success };
        var grants = new Dictionary<int, int>();
        int newCount = row.LivenessCount;

        foreach (var tid in req.TaskIds)
        {
            if (taken.Contains(tid)) continue;
            taken.Add(tid);
            resp.TaskIds.Add(tid);

            if (taskLookup.TryGetValue(tid, out var cfg))
            {
                foreach (var kv in cfg.TaskReward)
                {
                    if (kv.Key == 9)
                    {
                        newCount += kv.Value; // Liveness points
                    }
                    else
                    {
                        grants[kv.Key] = grants.GetValueOrDefault(kv.Key, 0) + kv.Value;
                    }
                }
            }
            else
            {
                newCount += 20;
            }
        }

        var updated = row with { LivenessCount = newCount, ClaimedTasks = new List<int>(taken) };
        SqliteStore.SaveLiveness(updated);

        var result = AkiResult.Ok(EMessageId.LivenessTaskTakeResponse, resp)
            .WithPush(EMessageId.LivenessCountUpdateNotify, new LivenessCountUpdateNotify { LivenessCount = newCount });
        foreach (var kv in grants) result.WithGrant(kv.Key, kv.Value);
        return result;
    }

    public static AkiResult HandleLivenessTakeRequest(int playerId, byte[] payload)
    {
        var req = new LivenessTakeRequest();
        if (payload.Length > 0) req.MergeFrom(payload);

        var row = SqliteStore.GetOrCreateLiveness(playerId);
        var claimedBoxes = new HashSet<int>(row.ClaimedBoxes);
        var boxLookup = new Dictionary<int, BinDataCache.LivenessBox>();
        foreach (var b in BinDataCache.LivenessBoxes) boxLookup[b.Id] = b;

        var resp = new LivenessTakeResponse { ErrorCode = ErrorCode.Success };
        var grants = new Dictionary<int, int>();

        foreach (var boxId in req.Ids)
        {
            if (!boxLookup.TryGetValue(boxId, out var box)) continue;
            if (row.LivenessCount < box.Goal || claimedBoxes.Contains(boxId)) continue;

            claimedBoxes.Add(boxId);
            resp.Ids.Add(boxId);

            var dropItems = BinDataCache.ResolveDrop(box.DropId);
            if (dropItems.Count == 0)
            {
                dropItems.Add((3, 20)); // Astrite fallback matching Zig liveness helper
                dropItems.Add((2, 3000));
            }
            foreach (var (itemId, cnt) in dropItems)
            {
                grants[itemId] = grants.GetValueOrDefault(itemId, 0) + cnt;
            }
        }

        if (resp.Ids.Count > 0)
        {
            SqliteStore.SaveLiveness(row with { ClaimedBoxes = new List<int>(claimedBoxes) });
        }

        var result = AkiResult.Ok(EMessageId.LivenessTakeResponse, resp);
        foreach (var kv in grants) result.WithGrant(kv.Key, kv.Value);
        return result;
    }
}
