const std = @import("std");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const Transaction = @import("../handlers.zig").Transaction;

pub fn onPrivateChatDataRequest(
    txn: *Transaction(pb.PrivateChatDataRequest),
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    const chat_history: std.ArrayList(pb.PrivateChatHistoryContentProto) = .empty;
    try txn.conn.push(alloc.gpa, io, pb.PrivateChatHistoryNotify{ .AllChats = chat_history });
    txn.respond(.{ .LoadSucc = true });
}

pub fn onPrivateChatHistoryRequest(
    txn: *Transaction(pb.PrivateChatHistoryRequest),
) !void {
    txn.respond(.{
        .ErrorCode = .Success,
        .Data = .{
            .TargetUid = txn.message.TargetUid,
            .Chats = .empty,
            .HistoryIsEnd = true,
            .TotalNums = 0,
        },
    });
}

pub fn onPrivateChatRequest(
    txn: *Transaction(pb.PrivateChatRequest),
) !void {
    txn.respond(.{ .TargetUid = txn.message.TargetUid, .FilterMsg = txn.message.Content });
}

pub fn onChatRequest(
    txn: *Transaction(pb.ChatRequest),
) !void {
    _ = txn.message;
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onChannelChatRequest(
    txn: *Transaction(pb.ChannelChatRequest),
) !void {
    _ = txn.message;
    txn.respond(.{ .ErrorCode = .Success });
}
