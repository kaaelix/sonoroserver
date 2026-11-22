const GuideInfo = @This();
const std = @import("std");

const Allocator = std.mem.Allocator;

pub const default: GuideInfo = .{};
pub const data_path = "guide_info";

pub const TutorialRecord = struct {
    id: i32,
    create_time: u32 = 1,
    get_award: bool = false,
};

finished_groups: []i32 = &.{},
unlocked_tutorials: []TutorialRecord = &.{},

pub fn hasFinished(info: GuideInfo, group_id: i32) bool {
    if (group_id == 0) return true;
    for (info.finished_groups) |id| {
        if (id == group_id) return true;
    }
    return false;
}

pub fn addFinished(info: *GuideInfo, gpa: Allocator, group_id: i32) !void {
    if (hasFinished(info.*, group_id)) return;

    const new_groups = try gpa.alloc(i32, info.finished_groups.len + 1);
    @memcpy(new_groups[0..info.finished_groups.len], info.finished_groups);
    new_groups[info.finished_groups.len] = group_id;

    if (info.finished_groups.len != 0) gpa.free(info.finished_groups);
    info.finished_groups = new_groups;
}

pub fn findTutorialPtr(info: *GuideInfo, tutorial_id: i32) ?*TutorialRecord {
    for (info.unlocked_tutorials) |*rec| {
        if (rec.id == tutorial_id) return rec;
    }
    return null;
}

pub fn unlockTutorial(info: *GuideInfo, gpa: Allocator, tutorial_id: i32, create_time: u32) !TutorialRecord {
    if (findTutorialPtr(info, tutorial_id)) |rec| return rec.*;

    const rec: TutorialRecord = .{
        .id = tutorial_id,
        .create_time = if (create_time > 0) create_time else 1,
        .get_award = false,
    };
    const next = try gpa.alloc(TutorialRecord, info.unlocked_tutorials.len + 1);
    @memcpy(next[0..info.unlocked_tutorials.len], info.unlocked_tutorials);
    next[info.unlocked_tutorials.len] = rec;
    if (info.unlocked_tutorials.len != 0) gpa.free(info.unlocked_tutorials);
    info.unlocked_tutorials = next;
    return rec;
}

pub fn deinit(info: GuideInfo, gpa: Allocator) void {
    std.zon.parse.free(gpa, info);
}
