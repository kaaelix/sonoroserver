using System;
using System.Collections.Generic;
using System.IO;
using System.Net;
using System.Net.Sockets;
using System.Threading;
using System.Threading.Tasks;
using Aki.Protocol;
using Google.Protobuf;

namespace AkiLogicService;

public class IpcServer
{
    private readonly string _socketPath;
    private readonly int _tcpPort;
    private Socket? _unixListener;
    private TcpListener? _tcpListener;
    private bool _running;

    private static readonly Dictionary<ushort, Func<int, byte[], AkiResult>> Handlers = BuildHandlers();

    private static Dictionary<ushort, Func<int, byte[], AkiResult>> BuildHandlers()
    {
        var map = new Dictionary<ushort, Func<int, byte[], AkiResult>>
        {
            // Activity (SQLite claims backed); TowerGuide handled by gamesv tower.zig
            { 16076, ActivityHandler.HandleActivityRequest },
            { 17864, ActivityHandler.HandleActivityUpdateNotify },

            // BattlePass Full (SQLite + BinData BattlePassReward.json grants + ExpUpdateNotify push)
            { 22649, BattlePassHandler.HandleBattlePassRequest },
            { 22950, BattlePassHandler.HandleBattlePassTaskRequest },
            { 27378, BattlePassHandler.HandleBattlePassTakeRewardRequest },
            { 21032, BattlePassHandler.HandleBattlePassTakeAllRewardRequest },
            { 26786, BattlePassHandler.HandleBattlePassLevelUpRequest },
            { 29679, BattlePassHandler.HandleBattlePassRecurringTakeRequest },
            { 29651, StoreCardHandler.HandleBattlePassTaskTake },

            // MonthCard, WeekCard & SignActivity (SQLite + IPC v3 grants)
            { 22235, StoreCardHandler.HandleMonthCard },
            { 26823, StoreCardHandler.HandleWeekCardInfo },
            { 18381, StoreCardHandler.HandleWeekCardReward },
            { 29384, StoreCardHandler.HandleSignActivity },
            { 27704, StoreCardHandler.HandleSignActivityFreeDrop },

            // Liveness / Daily Activity (SQLite + Liveness.json + DropPackage grants + LivenessCountUpdateNotify push)
            { 19066, LivenessHandler.HandleLivenessRequest },
            { 29864, LivenessHandler.HandleLivenessTaskTakeRequest },
            { 24082, LivenessHandler.HandleLivenessTakeRequest },

            // Achievements (SQLite + Achievement/Group BinData + DropPackage grants + AchievementProgressNotify push)
            { 22315, AchievementHandler.HandleAchievementInfoRequest },
            { 28975, AchievementHandler.HandleAchievementUpdateRequest },
            { 27912, AchievementHandler.HandleAchievementFinishRequest },
            { 16062, AchievementHandler.HandleAchievementReceiveRequest },
            { 28143, AchievementHandler.HandleMulAchievementReceiveRequest },

            // Settings, AccessPath, Titles & Music (SQLite + BinData)
            { 22740, SettingsGuideHandler.HandleLanguageSettingUpdate },
            { 28228, SettingsGuideHandler.HandleInputSettingRequest },
            { 20344, SettingsGuideHandler.HandleInputSettingUpdate },
            { 28506, SettingsGuideHandler.HandleAccessPathConfig },
            { 25420, SettingsGuideHandler.HandlePlayerTitleRequest },
            { 16180, SettingsGuideHandler.HandlePhantomMusicInfo },
            { 25144, SettingsGuideHandler.HandlePhantomMusicUnlock },
            { 23417, SettingsGuideHandler.HandlePhantomMusicSwitch },
            { 18730, SettingsGuideHandler.HandlePhantomMusicCollectUpdate },
            { 17072, SettingsGuideHandler.HandlePhantomMusicSave },

            // Adventure & PayShop DirectBuy
            { 28237, StoreCardHandler.HandlePayShopDirectBuy },
            { 23421, StoreCardHandler.HandlePayInfoRequest },
            { 21361, StoreCardHandler.HandlePayGiftInfoRequest },
            { 28163, SettingsGuideHandler.HandleMonsterDetectionInfoRequest },
            { 26242, StoreCardHandler.HandleGetAdventureReward },

            // Ping / Health
            { 0, (_, _) => AkiResult.OkRaw(0, new byte[] { 1 }) }
        };
        ExtraHandlers.RegisterAll(map);
        return map;
    }

    public static int RichHandlerCount => Handlers.Count - 1;

    public IpcServer(string socketPath = "/tmp/aki_logic.sock", int tcpPort = 19999)
    {
        _socketPath = socketPath;
        _tcpPort = tcpPort;
    }

    public async Task StartAsync(CancellationToken ct = default)
    {
        _running = true;

        try
        {
            if (File.Exists(_socketPath)) File.Delete(_socketPath);
            var endpoint = new UnixDomainSocketEndPoint(_socketPath);
            _unixListener = new Socket(AddressFamily.Unix, SocketType.Stream, ProtocolType.Unspecified);
            _unixListener.Bind(endpoint);
            _unixListener.Listen(128);
            Console.WriteLine($"[AkiLogicService] IPC Server v3 listening on Unix Domain Socket: {_socketPath} ({RichHandlerCount} rich handlers)");
            _ = Task.Run(() => AcceptUnixClientsAsync(ct), ct);
        }
        catch (Exception ex)
        {
            Console.WriteLine($"[AkiLogicService] Unix socket init warning: {ex.Message}");
        }

        try
        {
            _tcpListener = new TcpListener(IPAddress.Loopback, _tcpPort);
            _tcpListener.Start(128);
            Console.WriteLine($"[AkiLogicService] IPC Server v3 listening on TCP Loopback: 127.0.0.1:{_tcpPort}");
            _ = Task.Run(() => AcceptTcpClientsAsync(ct), ct);
        }
        catch (Exception ex)
        {
            Console.WriteLine($"[AkiLogicService] TCP init error: {ex.Message}");
        }

        var tcs = new TaskCompletionSource();
        ct.Register(() => tcs.TrySetResult());
        await tcs.Task;
    }

    private async Task AcceptUnixClientsAsync(CancellationToken ct)
    {
        while (_running && !ct.IsCancellationRequested && _unixListener != null)
        {
            try
            {
                var client = await _unixListener.AcceptAsync(ct);
                _ = Task.Run(() => HandleClientAsync(new NetworkStream(client, ownsSocket: true), ct), ct);
            }
            catch (OperationCanceledException) { break; }
            catch (Exception ex)
            {
                if (!_running) break;
                Console.WriteLine($"[AkiLogicService] Unix accept error: {ex.Message}");
            }
        }
    }

    private async Task AcceptTcpClientsAsync(CancellationToken ct)
    {
        while (_running && !ct.IsCancellationRequested && _tcpListener != null)
        {
            try
            {
                var client = await _tcpListener.AcceptTcpClientAsync(ct);
                _ = Task.Run(() => HandleClientAsync(client.GetStream(), ct), ct);
            }
            catch (OperationCanceledException) { break; }
            catch (Exception ex)
            {
                if (!_running) break;
                Console.WriteLine($"[AkiLogicService] TCP accept error: {ex.Message}");
            }
        }
    }

    private async Task HandleClientAsync(Stream stream, CancellationToken ct)
    {
        using (stream)
        {
            var headerBuf = new byte[11];

            while (!ct.IsCancellationRequested)
            {
                try
                {
                    int readHeader = await ReadExactAsync(stream, headerBuf, 11, ct);
                    if (readHeader < 11) break;

                    int playerId = BitConverter.ToInt32(headerBuf, 0);
                    byte flags = headerBuf[4];
                    ushort msgId = BitConverter.ToUInt16(headerBuf, 5);
                    uint payloadLen = BitConverter.ToUInt32(headerBuf, 7);

                    var payload = payloadLen > 0 ? new byte[payloadLen] : Array.Empty<byte>();
                    if (payloadLen > 0)
                    {
                        int readPayload = await ReadExactAsync(stream, payload, (int)payloadLen, ct);
                        if (readPayload < payloadLen) break;
                    }

                    AkiResult result = AkiResult.Unhandled;
                    if (Handlers.TryGetValue(msgId, out var handler))
                    {
                        result = handler(playerId, payload);
                    }
                    else if (flags == 0 && DomainEngineHandler.TryHandle(playerId, msgId, payload, out var domainRes))
                    {
                        result = domainRes;
                    }

                    await WriteIpcV3ResponseAsync(stream, result, ct);
                }
                catch (Exception ex)
                {
                    Console.WriteLine($"[AkiLogicService] Connection handler error: {ex.Message}");
                    break;
                }
            }
        }
    }

    private static async Task WriteIpcV3ResponseAsync(Stream stream, AkiResult res, CancellationToken ct)
    {
        // [2B respMsgId][4B respLen][respBytes]
        var respHeader = new byte[6];
        BitConverter.TryWriteBytes(respHeader.AsSpan(0, 2), res.RespMsgId);
        BitConverter.TryWriteBytes(respHeader.AsSpan(2, 4), (uint)res.RespBytes.Length);
        await stream.WriteAsync(respHeader, ct);
        if (res.RespBytes.Length > 0) await stream.WriteAsync(res.RespBytes, ct);

        // [2B pushCount] + pushCount * ([2B pushMsgId][4B pushLen][pushBytes])
        var cntBuf = new byte[2];
        BitConverter.TryWriteBytes(cntBuf, (ushort)res.Pushes.Count);
        await stream.WriteAsync(cntBuf, ct);
        foreach (var (pId, pBytes) in res.Pushes)
        {
            var pHdr = new byte[6];
            BitConverter.TryWriteBytes(pHdr.AsSpan(0, 2), pId);
            BitConverter.TryWriteBytes(pHdr.AsSpan(2, 4), (uint)pBytes.Length);
            await stream.WriteAsync(pHdr, ct);
            if (pBytes.Length > 0) await stream.WriteAsync(pBytes, ct);
        }

        // [2B grantCount] + grantCount * ([4B itemId][4B count])
        BitConverter.TryWriteBytes(cntBuf, (ushort)res.Grants.Count);
        await stream.WriteAsync(cntBuf, ct);
        if (res.Grants.Count > 0)
        {
            var gBuf = new byte[8];
            foreach (var (itemId, count) in res.Grants)
            {
                BitConverter.TryWriteBytes(gBuf.AsSpan(0, 4), itemId);
                BitConverter.TryWriteBytes(gBuf.AsSpan(4, 4), count);
                await stream.WriteAsync(gBuf, ct);
            }
        }

        await stream.FlushAsync(ct);
    }

    private static async Task<int> ReadExactAsync(Stream s, byte[] buf, int count, CancellationToken ct)
    {
        int total = 0;
        while (total < count)
        {
            int r = await s.ReadAsync(buf.AsMemory(total, count - total), ct);
            if (r <= 0) return total;
            total += r;
        }
        return total;
    }

    public void Stop()
    {
        _running = false;
        _unixListener?.Dispose();
        _tcpListener?.Stop();
        if (File.Exists(_socketPath))
        {
            try { File.Delete(_socketPath); } catch { }
        }
    }
}
