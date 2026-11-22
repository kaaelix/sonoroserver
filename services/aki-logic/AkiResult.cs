using System;
using System.Collections.Generic;
using Aki.Protocol;
using Google.Protobuf;

namespace AkiLogicService;

public sealed class AkiResult
{
    public ushort RespMsgId { get; set; }
    public byte[] RespBytes { get; set; } = Array.Empty<byte>();
    public List<(ushort MsgId, byte[] Bytes)> Pushes { get; } = new();
    public List<(int ItemId, int Count)> Grants { get; } = new();

    public static AkiResult Unhandled => new();

    public static AkiResult Ok(EMessageId respId, IMessage msg) => new()
    {
        RespMsgId = (ushort)respId,
        RespBytes = msg.ToByteArray()
    };

    public static AkiResult OkRaw(ushort respId, byte[] bytes) => new()
    {
        RespMsgId = respId,
        RespBytes = bytes
    };

    public AkiResult WithPush(EMessageId msgId, IMessage msg)
    {
        Pushes.Add(((ushort)msgId, msg.ToByteArray()));
        return this;
    }

    public AkiResult WithGrant(int itemId, int count)
    {
        if (itemId > 0 && count > 0)
        {
            Grants.Add((itemId, count));
        }
        return this;
    }

    public AkiResult WithGrants(IEnumerable<(int ItemId, int Count)> items)
    {
        foreach (var (id, cnt) in items)
        {
            WithGrant(id, cnt);
        }
        return this;
    }
}
