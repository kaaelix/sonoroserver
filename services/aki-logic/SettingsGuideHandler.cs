using System;
using System.Collections.Generic;
using System.Text.Json;
using Aki.Protocol;
using Google.Protobuf;

namespace AkiLogicService;

public static class SettingsGuideHandler
{
    public static AkiResult HandleLanguageSettingUpdate(int playerId, byte[] payload)
    {
        var req = new LanguageSettingUpdateRequest();
        if (payload.Length > 0) req.MergeFrom(payload);

        if (playerId > 0)
        {
            SqliteStore.SetKv(playerId, "settings", "language", req.Language.ToString());
        }
        return AkiResult.Ok(EMessageId.LanguageSettingUpdateResponse, new LanguageSettingUpdateResponse { ErrorCode = ErrorCode.Success });
    }

    public static AkiResult HandleInputSettingRequest(int playerId, byte[] payload)
    {
        var resp = new InputSettingResponse();
        if (playerId > 0)
        {
            var b64 = SqliteStore.GetKv(playerId, "settings", "input_pb_b64");
            if (!string.IsNullOrEmpty(b64))
            {
                try
                {
                    var bytes = Convert.FromBase64String(b64);
                    var data = new InputSettingData();
                    data.MergeFrom(bytes);
                    resp.InputSettingData = data;
                }
                catch { }
            }
        }
        return AkiResult.Ok(EMessageId.InputSettingResponse, resp);
    }

    public static AkiResult HandleInputSettingUpdate(int playerId, byte[] payload)
    {
        var req = new InputSettingUpdateRequest();
        if (payload.Length > 0) req.MergeFrom(payload);

        if (playerId > 0 && req.InputSettingData != null)
        {
            var b64 = Convert.ToBase64String(req.InputSettingData.ToByteArray());
            SqliteStore.SetKv(playerId, "settings", "input_pb_b64", b64);
        }
        return AkiResult.Ok(EMessageId.InputSettingUpdateResponse, new InputSettingUpdateResponse { ErrorCode = ErrorCode.Success });
    }

    public static AkiResult HandleAccessPathConfig(int playerId, byte[] payload)
    {
        var resp = new AccessPathConfigResponse();
        foreach (var id in BinDataCache.AccessPathIds)
        {
            resp.Configs.Add(new AccessPathConfig
            {
                Id = id,
                BeginTime = 1,
                EndTime = 2_000_000_000
            });
        }
        return AkiResult.Ok(EMessageId.AccessPathConfigResponse, resp);
    }

    public static AkiResult HandlePlayerTitleRequest(int playerId, byte[] payload)
    {
        var resp = new PlayerTitleResponse { ErrorCode = ErrorCode.Success };
        foreach (var tid in BinDataCache.PlayerTitleIds)
        {
            resp.PlayerTitleInfo.Add(new PlayerTitleInfo
            {
                PlayerTitleId = tid,
                IsUnlock = true,
                UnlockTime = 1
            });
        }
        return AkiResult.Ok(EMessageId.PlayerTitleResponse, resp);
    }

    public static AkiResult HandlePhantomMusicInfo(int playerId, byte[] payload)
    {
        var resp = new PhantomMusicInfoResponse { ErrorCode = ErrorCode.Success };

        if (playerId > 0)
        {
            var curStr = SqliteStore.GetKv(playerId, "music", "cur_id");
            if (int.TryParse(curStr, out int curId)) resp.CurMusicId = curId;

            var favJson = SqliteStore.GetKv(playerId, "music", "collect_ids");
            if (!string.IsNullOrEmpty(favJson))
            {
                var favIds = JsonSerializer.Deserialize<List<int>>(favJson) ?? new();
                foreach (var mid in favIds)
                {
                    if (BinDataCache.AllMusicIds.Contains(mid)) resp.CollectMusicIds.Add(mid);
                }
            }
        }

        foreach (var kv in BinDataCache.PhonographAlbums)
        {
            var alb = new PhonographAlbumTimeInfo
            {
                AlbumId = kv.Key,
                BeginTime = 0,
                EndTime = 4070908800
            };
            foreach (var mid in kv.Value) alb.MusicIds.Add(mid);
            resp.AlbumTimeInfos.Add(alb);
        }

        return AkiResult.Ok(EMessageId.PhantomMusicInfoResponse, resp);
    }

    public static AkiResult HandlePhantomMusicUnlock(int playerId, byte[] payload)
    {
        var req = new PhantomMusicUnlockRequest();
        if (payload.Length > 0) req.MergeFrom(payload);

        var resp = new PhantomMusicUnlockResponse { ErrorCode = ErrorCode.Success };
        foreach (var mid in BinDataCache.AllMusicIds) resp.AddMusicIds.Add(mid);
        return AkiResult.Ok(EMessageId.PhantomMusicUnlockResponse, resp);
    }

    public static AkiResult HandlePhantomMusicSwitch(int playerId, byte[] payload)
    {
        var req = new PhantomMusicSwitchRequest();
        if (payload.Length > 0) req.MergeFrom(payload);

        if (playerId > 0) SqliteStore.SetKv(playerId, "music", "cur_id", req.MusicId.ToString());
        return AkiResult.Ok(EMessageId.PhantomMusicSwitchResponse, new PhantomMusicSwitchResponse { ErrorCode = ErrorCode.Success });
    }

    public static AkiResult HandlePhantomMusicCollectUpdate(int playerId, byte[] payload)
    {
        var req = new PhantomMusicCollectUpdateRequest();
        if (payload.Length > 0) req.MergeFrom(payload);

        if (playerId > 0)
        {
            var valid = new List<int>();
            foreach (var mid in req.CollectMusicIds)
            {
                if (BinDataCache.AllMusicIds.Contains(mid) && !valid.Contains(mid)) valid.Add(mid);
            }
            SqliteStore.SetKv(playerId, "music", "collect_ids", JsonSerializer.Serialize(valid));
        }
        return AkiResult.Ok(EMessageId.PhantomMusicCollectUpdateResponse, new PhantomMusicCollectUpdateResponse { ErrorCode = ErrorCode.Success });
    }

    public static AkiResult HandlePhantomMusicSave(int playerId, byte[] payload)
    {
        return AkiResult.Ok(EMessageId.PhantomMusicSaveResponse, new PhantomMusicSaveResponse { ErrCode = ErrorCode.Success });
    }

    public static AkiResult HandleMonsterDetectionInfoRequest(int playerId, byte[] payload)
    {
        var resp = new MonsterDetectionInfoResponse();
        foreach (int id in BinDataCache.MonsterDetectionIds)
        {
            resp.DetectionTarget.Add(new DetectionTarget
            {
                Id = id,
                Type = DetectionType.NormalMonster,
                UnlockState = true,
                RefresherTime = 0,
                DetectionId = id
            });
        }
        foreach (int id in BinDataCache.DungeonDetectionIds)
        {
            resp.DetectionTarget.Add(new DetectionTarget
            {
                Id = id,
                Type = DetectionType.Dungeon,
                UnlockState = true,
                RefresherTime = 0,
                DetectionId = id
            });
        }
        foreach (int id in BinDataCache.SilentAreaDetectionIds)
        {
            resp.DetectionTarget.Add(new DetectionTarget
            {
                Id = id,
                Type = DetectionType.SilentArea,
                UnlockState = true,
                RefresherTime = 0,
                DetectionId = id
            });
        }
        return AkiResult.Ok(EMessageId.MonsterDetectionInfoResponse, resp);
    }
}
