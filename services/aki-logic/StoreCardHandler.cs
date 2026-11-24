using System;
using System.Collections.Generic;
using Aki.Protocol;
using Google.Protobuf;

namespace AkiLogicService;

public static class StoreCardHandler
{
    public static AkiResult HandleBattlePassTaskTake(int playerId, byte[] payload)
    {
        var req = new BattlePassTaskTakeRequest();
        if (payload.Length > 0) req.MergeFrom(payload);

        var row = SqliteStore.GetOrCreateBattlePass(playerId);
        var takenSet = new HashSet<int>(row.TakenTasks);
        var finishedSet = SqliteStore.GetFinishedBpTasks(playerId);
        var resp = new BattlePassTaskTakeResponse { ErrorCode = ErrorCode.Success };
        var grants = new Dictionary<int, int>();
        int gainedExp = 0;

        foreach (var tid in req.Ids)
        {
            if (takenSet.Contains(tid)) continue;
            if (!BinDataCache.BattlePassTasks.TryGetValue(tid, out var cfg)) continue;
            if (cfg.UpdateType != 1 && !finishedSet.Contains(tid)) continue;

            takenSet.Add(tid);
            resp.Ids.Add(tid);
            foreach (var kv in cfg.TaskReward)
            {
                if (kv.Key == 8)
                {
                    gainedExp += kv.Value;
                }
                else if (kv.Key > 0 && kv.Value > 0)
                {
                    grants[kv.Key] = grants.GetValueOrDefault(kv.Key, 0) + kv.Value;
                }
            }
        }

        int newLevel = row.Level;
        int newExp = row.Exp + gainedExp;
        int newWeekly = row.WeeklyExp + gainedExp;
        while (newExp >= 1000 && newLevel < 70)
        {
            newExp -= 1000;
            newLevel++;
        }

        if (resp.Ids.Count > 0)
        {
            SqliteStore.SaveBattlePass(row with
            {
                Level = newLevel,
                Exp = newExp,
                WeeklyExp = newWeekly,
                TakenTasks = new List<int>(takenSet)
            });
        }

        var notify = new BattlePassExpUpdateNotify
        {
            Level = newLevel,
            Exp = newExp,
            WeeklyTotalExp = newWeekly
        };

        var result = AkiResult.Ok(EMessageId.BattlePassTaskTakeResponse, resp)
            .WithPush(EMessageId.BattlePassExpUpdateNotify, notify);
        foreach (var kv in grants) result.WithGrant(kv.Key, kv.Value);
        return result;
    }

    public static AkiResult HandlePayShopDirectBuy(int playerId, byte[] payload)
    {
        var req = new PayShopDirectBuyRequest();
        if (payload.Length > 0) req.MergeFrom(payload);

        int shopItemId = req.Id;
        int itemId = BinDataCache.PayGiftItems.TryGetValue(shopItemId, out int giftItemId) ? giftItemId : 3;
        int itemCount = giftItemId > 0 ? 1 : 60;
        string receiptId = $"aki_direct_{playerId}_{shopItemId}_{DateTimeOffset.UtcNow.ToUnixTimeSeconds()}";

        if (playerId > 0 && shopItemId > 0)
        {
            SqliteStore.IncrementShopBought(playerId, 0, shopItemId, 1);
        }

        var resp = new PayShopDirectBuyResponse
        {
            ErrorCode = ErrorCode.Success,
            ReceiptId = receiptId
        };

        var notify = new PayShopDirectBuyNotify
        {
            ReceiptId = receiptId,
            ShopItemId = shopItemId,
            ItemId = itemId,
            ItemCount = itemCount
        };

        var result = AkiResult.Ok(EMessageId.PayShopDirectBuyResponse, resp)
            .WithPush(EMessageId.PayShopDirectBuyNotify, notify);
        if (shopItemId > 0) result.WithGrant(itemId, itemCount);
        return result;
    }

    public static AkiResult HandlePayInfoRequest(int playerId, byte[] payload)
    {
        var resp = new PayInfoResponse
        {
            Version = "3.8.11",
            ErrorCode = ErrorCode.Success
        };
        var boughtMap = playerId > 0 ? SqliteStore.GetShopBoughtCounts(playerId, 999) : new Dictionary<int, int>();
        foreach (var item in BinDataCache.PayItems)
        {
            bool canSpecial = item.SpecialBonusItemCount > 0 && !boughtMap.ContainsKey(item.Id);
            resp.Infos.Add(new PayItemInfo
            {
                Id = item.Id,
                PayId = item.PayId,
                ItemId = item.ItemId,
                ItemCount = item.ItemCount,
                BonusItemCount = item.BonusItemCount,
                SpecialBonusItemCount = item.SpecialBonusItemCount,
                CanSpecialBonus = canSpecial,
                StageImage = item.StageImage,
                ProductId = item.ProductId,
                Amount = item.Amount,
                Quality = 0
            });
        }
        return AkiResult.Ok(EMessageId.PayInfoResponse, resp);
    }

    public static AkiResult HandlePayGiftInfoRequest(int playerId, byte[] payload)
    {
        var boughtMap = playerId > 0 ? SqliteStore.GetShopBoughtCounts(playerId, 0) : new Dictionary<int, int>();
        var resp = new PayGiftInfoResponse
        {
            Version = "3.8.11",
            ErrorCode = ErrorCode.Success
        };
        foreach (var kv in BinDataCache.PayGiftItems)
        {
            int bought = boughtMap.TryGetValue(kv.Key, out int b) ? b : 0;
            resp.Infos.Add(new PayGiftInfo
            {
                Id = kv.Key,
                PayId = kv.Key,
                ItemId = kv.Value,
                ItemCount = 1,
                BuyLimit = 99,
                BoughtCount = bought,
                BeginTime = 1,
                EndTime = 2_000_000_000,
                IsCanBuy = true
            });
        }
        return AkiResult.Ok(EMessageId.PayGiftInfoResponse, resp);
    }

    public static AkiResult HandleGetAdventureReward(int playerId, byte[] payload)
    {
        var req = new GetAdventureRewardRequest();
        if (payload.Length > 0) req.MergeFrom(payload);

        int taskId = req.Id;
        var result = AkiResult.Ok(EMessageId.GetAdventureRewardResponse, new GetAdventureRewardResponse
        {
            Code = ErrorCode.Success,
            Id = taskId
        });

        if (playerId > 0 && taskId > 0)
        {
            string claimKey = $"adv_task:{taskId}";
            var claims = SqliteStore.GetActivityClaims(playerId);
            var (_, progClaims) = SqliteStore.GetProgressState(playerId);
            if (!claims.Contains(claimKey) && !progClaims.Contains(claimKey))
            {
                SqliteStore.SetActivityClaim(playerId, claimKey, "1");
                int dropId = BinDataCache.AdventureTaskDrops.TryGetValue(taskId, out int did) ? did : 0;
                var drops = BinDataCache.ResolveDrop(dropId);
                if (drops.Count == 0)
                {
                    drops.Add((3, 40));
                    drops.Add((1, 100));
                }
                result.WithGrants(drops);
            }
        }

        return result;
    }

    public static AkiResult HandleMonthCard(int playerId, byte[] payload)
    {
        long dayBucket = DateTimeOffset.UtcNow.ToUnixTimeSeconds() / 86_400;
        string? lastDayStr = playerId > 0 ? SqliteStore.GetKv(playerId, "wallet", "month_card_last_day") : null;
        bool alreadyGot = long.TryParse(lastDayStr, out long lastDay) && lastDay == dayBucket;

        var result = AkiResult.Ok(EMessageId.MonthCardResponse, new MonthCardResponse
        {
            Days = 30,
            IsDailyGot = true,
            ErrorCode = ErrorCode.Success
        });

        if (!alreadyGot && playerId > 0)
        {
            SqliteStore.SetKv(playerId, "wallet", "month_card_last_day", dayBucket.ToString());
            result.WithGrant(3, 90); // 90 Astrites daily
        }

        return result;
    }

    public static AkiResult HandleWeekCardInfo(int playerId, byte[] payload)
    {
        var req = new WeekCardInfoRequest();
        if (payload.Length > 0) req.MergeFrom(payload);

        long nowS = DateTimeOffset.UtcNow.ToUnixTimeSeconds();
        var claims = playerId > 0 ? SqliteStore.GetActivityClaims(playerId) : new HashSet<string>();

        var resp = new WeekCardInfoResponse { ErrorCode = ErrorCode.Success };
        foreach (var wid in req.WeekCardIds)
        {
            var info = new WeekCardInfo
            {
                WeekCardId = wid,
                Days = 7,
                BuyTimeStamp = nowS,
                EndTimeStamp = nowS + 7 * 86_400
            };
            for (int day = 1; day <= 3; day++)
            {
                int cid = wid * 100 + day;
                info.WeekCardContentInfos.Add(new WeekCardContentInfo
                {
                    WeekCardContentId = cid,
                    UnlockDay = day,
                    UnlockTimeStamp = nowS,
                    Status = claims.Contains($"week_card:{cid}") ? 2 : 1
                });
            }
            resp.WeekCardInfos.Add(info);
        }

        resp.Infos = resp.WeekCardInfos.Count > 0
            ? resp.WeekCardInfos[0].Clone()
            : new WeekCardInfo
            {
                WeekCardId = req.WeekCardId,
                Days = 7,
                BuyTimeStamp = nowS,
                EndTimeStamp = nowS + 7 * 86_400
            };

        return AkiResult.Ok(EMessageId.WeekCardInfoResponse, resp);
    }

    public static AkiResult HandleWeekCardReward(int playerId, byte[] payload)
    {
        var req = new WeekCardRewardRequest();
        if (payload.Length > 0) req.MergeFrom(payload);

        var result = AkiResult.Ok(EMessageId.WeekCardRewardResponse, new WeekCardRewardResponse
        {
            ErrorCode = ErrorCode.Success
        });

        if (playerId > 0 && req.WeekCardContentId > 0)
        {
            string key = $"week_card:{req.WeekCardContentId}";
            var claims = SqliteStore.GetActivityClaims(playerId);
            if (!claims.Contains(key))
            {
                SqliteStore.SetActivityClaim(playerId, key, "1");
                result.WithGrant(3, 40).WithGrant(2, 10000);
            }
        }

        return result;
    }

    public static AkiResult HandleSignActivity(int playerId, byte[] payload)
    {
        var req = new SignActivityRequest();
        if (payload.Length > 0) req.MergeFrom(payload);

        var resp = new SignActivityResponse { ErrorCode = ErrorCode.Success };
        var result = AkiResult.Ok(EMessageId.SignActivityResponse, resp);

        if (playerId > 0)
        {
            string key = $"sign_act:{req.ActivityId}:{req.Index}";
            var claims = SqliteStore.GetActivityClaims(playerId);
            if (!claims.Contains(key))
            {
                SqliteStore.SetActivityClaim(playerId, key, "1");
                resp.ItemMap[3] = 30;
                resp.ItemMap[2] = 10000;
                result = AkiResult.Ok(EMessageId.SignActivityResponse, resp)
                    .WithGrant(3, 30)
                    .WithGrant(2, 10000);
            }
        }

        return result;
    }

    public static AkiResult HandleSignActivityFreeDrop(int playerId, byte[] payload)
    {
        var req = new SignActivityFreeDropRequest();
        if (payload.Length > 0) req.MergeFrom(payload);

        var result = AkiResult.Ok(EMessageId.SignActivityFreeDropResponse, new SignActivityFreeDropResponse
        {
            ErrorCode = ErrorCode.Success
        });

        if (playerId > 0)
        {
            string key = $"sign_free:{req.ActivityId}";
            var claims = SqliteStore.GetActivityClaims(playerId);
            if (!claims.Contains(key))
            {
                SqliteStore.SetActivityClaim(playerId, key, "1");
                result.WithGrant(3, 60).WithGrant(2, 20000);
            }
        }

        return result;
    }
}
