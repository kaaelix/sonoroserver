using System;
using System.Collections;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Reflection;
using System.Text.Json;
using Aki.Protocol;
using Google.Protobuf;

namespace AkiLogicService;

public static class DomainEngineHandler
{
    private record RouteEntry(int req_id, string req_name, int resp_id, string resp_name, string domain);

    private sealed class CompiledRoute
    {
        public ushort ReqId { get; init; }
        public string ReqName { get; init; } = "";
        public ushort RespId { get; init; }
        public string RespName { get; init; } = "";
        public string Domain { get; init; } = "";
        public Type? ReqType { get; init; }
        public Type RespType { get; init; } = typeof(object);
        public bool IsReward { get; init; }
        public bool IsMutation { get; init; }
    }

    private static readonly Dictionary<ushort, CompiledRoute> Routes = new();

    static DomainEngineHandler()
    {
        LoadRoutes();
    }

    public static int LoadedRouteCount => Routes.Count;

    private static void LoadRoutes()
    {
        try
        {
            string? path = new[]
            {
                Path.Combine(AppContext.BaseDirectory, "domain_routes.json"),
                Path.Combine(BinDataCache.RepoRoot, "services", "aki-logic", "domain_routes.json")
            }.FirstOrDefault(File.Exists);

            if (path == null)
            {
                Console.WriteLine("[DomainEngineHandler] Warning: domain_routes.json not found.");
                return;
            }

            var json = File.ReadAllText(path);
            var list = JsonSerializer.Deserialize<List<RouteEntry>>(json) ?? new();

            var protoAsm = typeof(BattlePassRequest).Assembly;
            Type[] allTypes;
            try
            {
                allTypes = protoAsm.GetTypes();
            }
            catch (ReflectionTypeLoadException ex)
            {
                allTypes = ex.Types.Where(t => t != null).Cast<Type>().ToArray();
            }

            var msgIdToType = new Dictionary<ushort, Type>();
            foreach (var t in allTypes)
            {
                try
                {
                    if (!typeof(IMessage).IsAssignableFrom(t) || t.IsAbstract || t.IsInterface) continue;
                    var attr = t.GetCustomAttribute<NetMessageAttribute>();
                    if (attr != null) msgIdToType[(ushort)attr.MessageId] = t;
                }
                catch { }
            }

            foreach (var item in list)
            {
                ushort reqId = (ushort)item.req_id;
                ushort respId = (ushort)item.resp_id;
                if (!msgIdToType.TryGetValue(respId, out var respType)) continue;
                msgIdToType.TryGetValue(reqId, out var reqType);

                string rn = item.req_name;
                bool isLockOrRefresh = rn.Contains("Lock", StringComparison.OrdinalIgnoreCase) ||
                                       rn.Contains("Refresh", StringComparison.OrdinalIgnoreCase);
                bool isReward = !isLockOrRefresh && (
                                rn.Contains("Reward", StringComparison.OrdinalIgnoreCase) ||
                                rn.Contains("Receive", StringComparison.OrdinalIgnoreCase) ||
                                rn.Contains("FreeDrop", StringComparison.OrdinalIgnoreCase) ||
                                rn.Contains("Award", StringComparison.OrdinalIgnoreCase) ||
                                rn.Contains("Claim", StringComparison.OrdinalIgnoreCase) ||
                                rn.EndsWith("TakeRequest", StringComparison.OrdinalIgnoreCase) ||
                                rn.EndsWith("SettleRequest", StringComparison.OrdinalIgnoreCase));

                bool isMut = isReward ||
                             rn.Contains("Save", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Update", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Select", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Set", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Modify", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Add", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Delete", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Equip", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Unlock", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Finish", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Start", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Reset", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Refresh", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Read", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Operate", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Apply", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Put", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Change", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Rename", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Buy", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Upgrade", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Build", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("LevelUp", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Compose", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Sell", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Lock", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Enter", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Quit", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Over", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Fight", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Revive", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Pay", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("CdKey", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Drown", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("LandingDamage", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("BatchOper", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Train", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Transfer", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Challenge", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Switch", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Retry", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Advance", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Use", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Action", StringComparison.OrdinalIgnoreCase) ||
                             rn.Contains("Move", StringComparison.OrdinalIgnoreCase);

                Routes[reqId] = new CompiledRoute
                {
                    ReqId = reqId,
                    ReqName = rn,
                    RespId = respId,
                    RespName = item.resp_name,
                    Domain = item.domain,
                    ReqType = reqType,
                    RespType = respType,
                    IsReward = isReward,
                    IsMutation = isMut
                };
            }

            Console.WriteLine($"[DomainEngineHandler] Compiled {Routes.Count} SQLite-backed domain RPC routes.");
        }
        catch (Exception ex)
        {
            Console.WriteLine($"[DomainEngineHandler] Init error: {ex.Message}");
        }
    }

    public static bool TryHandle(int playerId, ushort msgId, byte[] payload, out AkiResult result)
    {
        if (!Routes.TryGetValue(msgId, out var route))
        {
            result = AkiResult.Unhandled;
            return false;
        }

        object? reqObj = null;
        if (route.ReqType != null)
        {
            try
            {
                reqObj = Activator.CreateInstance(route.ReqType);
                if (reqObj is IMessage reqPb && payload.Length > 0)
                {
                    reqPb.MergeFrom(payload);
                }
            }
            catch { }
        }

        var claimedIds = ExtractRequestIds(reqObj);
        int primaryId = claimedIds.Count > 0 ? claimedIds[0] : 0;

        // Persist mutation state directly to SQLite player_kv
        if (playerId > 0 && route.IsMutation && payload.Length > 0)
        {
            string kvKey = primaryId > 0 ? $"{route.ReqName}:{primaryId}" : route.ReqName;
            SqliteStore.SetKv(playerId, route.Domain, kvKey, Convert.ToBase64String(payload));
        }

        object? respObj = Activator.CreateInstance(route.RespType);
        if (respObj is not IMessage respPb)
        {
            result = AkiResult.Unhandled;
            return false;
        }

        SetSuccessErrorCode(route.RespType, respObj);
        InitializeSafeSubMessages(route.RespType, respObj);

        var res = new AkiResult { RespMsgId = route.RespId };
        var grantedItems = new Dictionary<int, int>();
        var newlyClaimed = new List<int>();

        if (route.IsReward && playerId > 0)
        {
            var existingClaims = SqliteStore.GetActivityClaims(playerId);
            var idsToCheck = claimedIds.Count > 0 ? claimedIds : new List<int> { 0 };

            foreach (var id in idsToCheck)
            {
                string claimKey = $"{route.Domain}:{route.ReqName}:{id}";
                if (existingClaims.Contains(claimKey)) continue;

                SqliteStore.SetActivityClaim(playerId, claimKey, "1");
                if (id > 0) newlyClaimed.Add(id);

                var drops = BinDataCache.ResolveDrop(id);
                if (drops.Count == 0)
                {
                    drops.Add((3, 40));    // Astrite
                    drops.Add((2, 15000)); // Shell Credit
                }
                foreach (var (itemId, cnt) in drops)
                {
                    grantedItems[itemId] = grantedItems.GetValueOrDefault(itemId, 0) + cnt;
                }
            }

            PopulateRewardResponseFields(route.RespType, respObj, newlyClaimed, grantedItems);
            foreach (var kv in grantedItems) res.WithGrant(kv.Key, kv.Value);
        }
        else if (primaryId > 0)
        {
            EchoPrimaryIdIfPresent(route.RespType, respObj, primaryId);
        }

        res.RespBytes = respPb.ToByteArray();
        result = res;
        return true;
    }

    private static List<int> ExtractRequestIds(object? reqObj)
    {
        var ids = new List<int>();
        if (reqObj == null) return ids;
        var t = reqObj.GetType();

        foreach (var listPropName in new[]
        {
            "Ids", "RewardIds", "TaskIds", "AchievementIds", "ClaimIds",
            "ChallengeIds", "ScoreIds", "TargetIds", "ConfigIds", "Indexs",
            "CollectIds", "Configs", "SelectedId", "RewardIdxs"
        })
        {
            var p = t.GetProperty(listPropName, BindingFlags.Public | BindingFlags.Instance);
            if (p != null && p.GetValue(reqObj) is IEnumerable enumerable)
            {
                foreach (var item in enumerable)
                {
                    if (item is int iv && iv > 0 && !ids.Contains(iv)) ids.Add(iv);
                }
                if (ids.Count > 0) return ids;
            }
        }

        foreach (var scalarName in new[]
        {
            "Id", "RewardId", "TaskId", "ActivityId", "Level", "GroupId",
            "ConfigId", "Index", "InstId", "SlotIndex",
            "TotalTopUpRewardId", "InfluenceId", "AreaSecretId", "LevelId",
            "ChallengeId", "WrapId", "PhantomBattleRoleId", "RoleId",
            "RoleSkinTrialInfoId", "GatherTaskId", "RoleInviteId", "DropCatchId",
            "HonamiStoryMascotId", "PreheatNodeId", "ScoreId", "SelectionId",
            "SubLevelId", "RequireId"
        })
        {
            var p = t.GetProperty(scalarName, BindingFlags.Public | BindingFlags.Instance);
            if (p != null && p.PropertyType == typeof(int))
            {
                int val = (int)(p.GetValue(reqObj) ?? 0);
                if (val > 0)
                {
                    ids.Add(val);
                    break;
                }
            }
        }
        return ids;
    }

    private static void SetSuccessErrorCode(Type respType, object respObj)
    {
        foreach (var name in new[] { "ErrorCode", "ErrCode", "Code" })
        {
            var prop = respType.GetProperty(name, BindingFlags.Public | BindingFlags.Instance);
            if (prop != null && prop.CanWrite && prop.PropertyType == typeof(ErrorCode))
            {
                prop.SetValue(respObj, ErrorCode.Success);
            }
        }
    }

    private static void InitializeSafeSubMessages(Type respType, object respObj)
    {
        bool hasOneof = respType.GetProperties(BindingFlags.Public | BindingFlags.Instance)
            .Any(p => p.PropertyType.IsEnum && p.Name.EndsWith("Case", StringComparison.Ordinal));
        if (hasOneof) return;

        foreach (var prop in respType.GetProperties(BindingFlags.Public | BindingFlags.Instance))
        {
            if (!prop.CanWrite || !prop.CanRead) continue;
            var pt = prop.PropertyType;
            if (pt == respType || pt == typeof(string)) continue;
            if (!typeof(IMessage).IsAssignableFrom(pt) || pt.IsAbstract || pt.IsInterface) continue;
            if (prop.GetValue(respObj) == null)
            {
                try
                {
                    prop.SetValue(respObj, Activator.CreateInstance(pt));
                }
                catch { }
            }
        }
    }

    private static void PopulateRewardResponseFields(
        Type respType,
        object respObj,
        List<int> claimedIds,
        Dictionary<int, int> grantedItems)
    {
        foreach (var listName in new[] { "Ids", "RewardIds", "SuccessIds", "ActiveEventIds", "ScoreRewardId", "TakeRewardResultIds" })
        {
            var p = respType.GetProperty(listName, BindingFlags.Public | BindingFlags.Instance);
            if (p != null && p.GetValue(respObj) is IList<int> list)
            {
                foreach (var id in claimedIds) list.Add(id);
            }
        }

        foreach (var mapName in new[] { "ItemMap", "RewardMap", "DropMap", "RewardItems" })
        {
            var p = respType.GetProperty(mapName, BindingFlags.Public | BindingFlags.Instance);
            if (p != null && p.GetValue(respObj) is IDictionary<int, int> map)
            {
                foreach (var kv in grantedItems) map[kv.Key] = kv.Value;
            }
        }

        if (claimedIds.Count == 1)
        {
            EchoPrimaryIdIfPresent(respType, respObj, claimedIds[0]);
        }
    }

    private static void EchoPrimaryIdIfPresent(Type respType, object respObj, int primaryId)
    {
        foreach (var propName in new[] { "Id", "ActivityId", "InstId", "SlotIndex", "GroupId" })
        {
            var p = respType.GetProperty(propName, BindingFlags.Public | BindingFlags.Instance);
            if (p != null && p.CanWrite && p.PropertyType == typeof(int))
            {
                p.SetValue(respObj, primaryId);
                return;
            }
        }
    }
}
