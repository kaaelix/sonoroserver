const std = @import("std");
const Extra = @import("Extra.zig");

BaseInfoComponent: ?@import("BaseInfo.zig") = null,
AiComponent: ?@import("Ai.zig") = null,
AttributeComponent: ?@import("Attribute.zig") = null,
ModelComponent: ?@import("Model.zig") = null,
MonsterComponent: ?@import("Monster.zig") = null,
EntityStateComponent: ?@import("EntityState.zig") = null,
RewardComponent: ?@import("Reward.zig") = null,
TreasureBoxComponent: ?@import("TreasureBox.zig") = null,
VisionCaptureComponent: ?@import("VisionCapture.zig") = null,
RangeComponent: ?@import("Range.zig") = null,
InteractComponent: ?@import("Interact.zig") = null,
TeleportComponent: ?@import("Teleport.zig") = null,
DynamicTeleportComponent: ?@import("Teleport.zig") = null,
SceneItemMovementComponent: ?@import("SceneItemMovement.zig") = null,
SplineComponent: ?@import("Spline.zig") = null,
VarComponent: ?@import("Var.zig") = null,
TriggerComponent: ?@import("Trigger.zig") = null,
ConditionListenerComponent: ?@import("ConditionListener.zig") = null,
RefreshComponent: ?@import("Refresh.zig") = null,
RefreshGroupComponent: ?@import("Refresh.zig") = null,
SceneItemLifeCycleComponent: ?@import("SceneItemLifeCycle.zig") = null,
LevelPlayComponent: ?@import("LevelPlay.zig") = null,
DestructibleItem: ?@import("DestructibleItem.zig") = null,
LiftComponent: ?@import("Lift.zig") = null,
PortalComponent: ?@import("Portal.zig") = null,
NoRenderPortalComponent: ?@import("Portal.zig") = null,
GravityFlipComponent: ?@import("GravityFlip.zig") = null,
CollectComponent: ?@import("Collect.zig") = null,
TeleControl2: ?@import("TeleControl2.zig") = null,
extra: []const Extra.Entry = &.{},

fn parseComponentPtrOrStruct(
    comptime T: type,
    allocator: std.mem.Allocator,
    source: anytype,
    options: std.json.ParseOptions,
) !T {
    const info = @typeInfo(T);
    if (info == .optional) {
        if (try source.peekNextTokenType() == .null) {
            _ = try source.next();
            return null;
        }
        return try parseComponentPtrOrStruct(info.optional.child, allocator, source, options);
    }
    if (info == .pointer and info.pointer.size == .one) {
        const ptr = try allocator.create(info.pointer.child);
        errdefer allocator.destroy(ptr);
        ptr.* = try parseComponentPtrOrStruct(info.pointer.child, allocator, source, options);
        return ptr;
    }
    if (info != .@"struct") {
        return try std.json.innerParse(T, allocator, source, options);
    }

    const has_disabled = comptime @hasField(T, "Disabled") and @FieldType(T, "Disabled") == ?bool;
    const has_growth = comptime @hasField(T, "MonsterPropGrowthId") and @FieldType(T, "MonsterPropGrowthId") == ?i32;
    const has_append_buffs = comptime @hasField(T, "AppendBuffIds") and @FieldType(T, "AppendBuffIds") == ?[]const i64;
    if (!has_disabled and !has_growth and !has_append_buffs) {
        return try std.json.innerParse(T, allocator, source, options);
    }

    if (.object_begin != try source.next()) return error.UnexpectedToken;
    var r: T = .{};
    const fields = comptime std.meta.fields(T);
    while (true) {
        const token = try source.nextAllocMax(allocator, .alloc_if_needed, options.max_value_len orelse std.json.default_max_value_len);
        const field_name = switch (token) {
            inline .string, .allocated_string => |slice| slice,
            .object_end => break,
            else => return error.UnexpectedToken,
        };
        var token_freed = false;
        errdefer if (!token_freed) switch (token) {
            .allocated_string => |slice| allocator.free(slice),
            else => {},
        };

        var matched = false;
        inline for (fields) |f| {
            if (!f.is_comptime and std.mem.eql(u8, f.name, field_name)) {
                switch (token) {
                    .allocated_string => |slice| allocator.free(slice),
                    else => {},
                }
                token_freed = true;
                // Client TdUtils.Merge drops template properties when an entity overrides them with JSON null.
                if (has_disabled and comptime std.mem.eql(u8, f.name, "Disabled")) {
                    const v = try std.json.innerParse(?bool, allocator, source, options);
                    r.Disabled = v orelse false;
                } else if (has_growth and comptime std.mem.eql(u8, f.name, "MonsterPropGrowthId")) {
                    const v = try std.json.innerParse(?i32, allocator, source, options);
                    r.MonsterPropGrowthId = v orelse 0;
                } else if (has_append_buffs and comptime std.mem.eql(u8, f.name, "AppendBuffIds")) {
                    const v = try std.json.innerParse(?[]const i64, allocator, source, options);
                    r.AppendBuffIds = v orelse &.{};
                } else {
                    @field(r, f.name) = try std.json.innerParse(f.type, allocator, source, options);
                }
                matched = true;
                break;
            }
        }
        if (matched) continue;

        switch (token) {
            .allocated_string => |slice| allocator.free(slice),
            else => {},
        }
        token_freed = true;
        if (options.ignore_unknown_fields) {
            try source.skipValue();
        } else {
            return error.UnknownField;
        }
    }
    return r;
}

pub fn jsonParse(allocator: std.mem.Allocator, source: anytype, options: std.json.ParseOptions) !@This() {
    if (.object_begin != try source.next()) return error.UnexpectedToken;
    var r: @This() = .{};
    var extra_list: std.ArrayListUnmanaged(Extra.Entry) = .empty;

    while (true) {
        const token = try source.nextAllocMax(allocator, .alloc_if_needed, options.max_value_len orelse std.json.default_max_value_len);
        const field_name = switch (token) {
            inline .string, .allocated_string => |slice| slice,
            .object_end => break,
            else => return error.UnexpectedToken,
        };
        defer switch (token) {
            .allocated_string => |slice| allocator.free(slice),
            else => {},
        };

        var matched = false;
        inline for (comptime std.meta.fields(@This())) |field| {
            if (!comptime std.mem.eql(u8, field.name, "extra")) {
                if (std.mem.eql(u8, field.name, field_name)) {
                    @field(r, field.name) = try parseComponentPtrOrStruct(field.type, allocator, source, options);
                    matched = true;
                    break;
                }
            }
        }
        if (matched) continue;

        inline for (comptime std.meta.fields(Extra.Entry)) |efield| {
            if (std.mem.eql(u8, efield.name, field_name)) {
                const ptr = try parseComponentPtrOrStruct(efield.type, allocator, source, options);
                try extra_list.append(allocator, @unionInit(Extra.Entry, efield.name, ptr));
                matched = true;
                break;
            }
        }
        if (matched) continue;

        if (options.ignore_unknown_fields) {
            try source.skipValue();
        } else {
            return error.UnknownField;
        }
    }
    if (extra_list.items.len > 0) {
        r.extra = try extra_list.toOwnedSlice(allocator);
    }
    return r;
}

fn mergeComponentField(comptime field_name: []const u8, template: *const @This(), target: *@This()) void {
    if (comptime std.mem.eql(u8, field_name, "extra")) {
        if (target.extra.len == 0 and template.extra.len > 0) target.extra = template.extra;
        return;
    }
    if (@field(target, field_name)) |*target_comp| {
        const template_comp = @field(template, field_name) orelse return;

        inline for (comptime std.meta.fields(@TypeOf(target_comp.*))) |field| {
            switch (@typeInfo(field.type)) {
                .optional => {
                    if (@field(target_comp, field.name) == null) {
                        @field(target_comp, field.name) = @field(template_comp, field.name);
                    }
                },
                else => {},
            }
        }
    } else {
        @field(target, field_name) = @field(template, field_name);
    }
}

pub fn mergeInto(template: *const @This(), target: *@This()) void {
    inline for (comptime std.meta.fields(@This())) |field| {
        mergeComponentField(field.name, template, target);
    }
    if (template.InteractComponent) |template_interact| {
        if (target.InteractComponent) |*target_interact| {
            if (target_interact.Options.len == 0 and template_interact.Options.len > 0) {
                target_interact.Options = template_interact.Options;
            }
            if (target_interact.RandomInteract) |*target_random| {
                if (template_interact.RandomInteract) |template_random| {
                    if (target_random.RandomCount == 0) target_random.RandomCount = template_random.RandomCount;
                    if (target_random.Options.len == 0 and template_random.Options.len > 0) {
                        target_random.Options = template_random.Options;
                    }
                }
            }
        }
    }
    if (template.VarComponent) |tv| {
        if (target.VarComponent) |*rv| {
            if (rv.Vars.len == 0 and tv.Vars.len > 0) rv.Vars = tv.Vars;
        }
    }
    if (template.GravityFlipComponent) |tg| {
        if (target.GravityFlipComponent) |*rg| {
            if (rg.Config.len == 0 and tg.Config.len > 0) rg.Config = tg.Config;
        }
    }
}

test "Components explicit null Disabled merge parity" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const aalloc = arena.allocator();
    const opts: std.json.ParseOptions = .{ .ignore_unknown_fields = true };
    const tpl = try std.json.parseFromSliceLeaky(@This(), aalloc,
        \\{"InteractComponent":{"Disabled":true,"Options":[{"Type":{"Type":"Actions","Actions":[{"Name":"ChangeSelfEntityState","Params":{"EntityState":"关卡.Common.状态.激活"}}]}}]}}
    , opts);
    var cleared = try std.json.parseFromSliceLeaky(@This(), aalloc,
        \\{"InteractComponent":{"Disabled":null}}
    , opts);
    tpl.mergeInto(&cleared);
    try std.testing.expect(!(cleared.InteractComponent.?.Disabled orelse false));
    try std.testing.expectEqual(@as(usize, 1), cleared.InteractComponent.?.Options.len);

    var omitted = try std.json.parseFromSliceLeaky(@This(), aalloc,
        \\{"InteractComponent":{}}
    , opts);
    tpl.mergeInto(&omitted);
    try std.testing.expect(omitted.InteractComponent.?.Disabled orelse false);
}
