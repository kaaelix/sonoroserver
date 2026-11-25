//! Durability of destructible scene items (the `DestructibleItem` entity component).
//!
//! The client only reports a hit (`ThrowDamageRequest`) while the durability it was given at
//! entity creation is above zero, so the value has to be sent with the entity. Every hit then
//! removes the amount the item's own configuration assigns to the attacker's weapon.
//!
//! Everything here is pure; values come from the static entity tables.

const std = @import("std");

/// Durability and per-hit damage the game assumes when an item's configuration gives none.
pub const default_durability: i32 = 100;
pub const default_hit_damage: i32 = 100;

/// Weapon families as named by the keys of `DestructibleItem.WeaponDamage`.
pub const WeaponKind = enum {
    great_sword,
    dagger,
    pistol,
    pugilism,
    ring,
    unknown,
};

/// Maps `RoleInfo.WeaponType` to the weapon family used by the damage configuration.
pub fn weaponKindFromRoleWeaponType(weapon_type: i32) WeaponKind {
    return switch (weapon_type) {
        1 => .great_sword,
        2 => .dagger,
        3 => .pistol,
        4 => .pugilism,
        5 => .ring,
        else => .unknown,
    };
}

/// Durability an item is created with: what the player's save still holds for it, otherwise the
/// configured value.
pub fn initialDurability(config_durability: ?i32, saved_durability: i64) i32 {
    if (saved_durability > 0) {
        if (std.math.cast(i32, saved_durability)) |saved| return saved;
    }
    const configured = config_durability orelse 0;
    return if (configured > 0) configured else default_durability;
}

/// Durability removed by one hit. `weapon_damage` is the item's `WeaponDamage` configuration (or
/// null), any struct with optional integer fields named after the configuration keys.
pub fn hitDamage(weapon_damage: anytype, kind: WeaponKind) i32 {
    const config = weapon_damage orelse return default_hit_damage;
    const for_weapon: ?i32 = switch (kind) {
        .great_sword => config.GreatSword,
        .dagger => config.Dagger,
        .pistol => config.Pistol,
        .pugilism => config.Pugilism,
        .ring => config.Ring,
        .unknown => null,
    };
    const damage = for_weapon orelse config.DefaultValue orelse default_hit_damage;
    return if (damage > 0) damage else default_hit_damage;
}

pub fn afterHit(current: i32, damage: i32) i32 {
    return @max(0, current - damage);
}

const TestWeaponDamage = struct {
    DefaultValue: ?i32 = null,
    GreatSword: ?i32 = null,
    Dagger: ?i32 = null,
    Pugilism: ?i32 = null,
    Pistol: ?i32 = null,
    Ring: ?i32 = null,
};

/// `WeaponDamage` of the ore blueprint `Collect504`, as in the template table.
const ore_weapon_damage: ?TestWeaponDamage = .{
    .DefaultValue = 100,
    .GreatSword = 150,
    .Dagger = 100,
    .Pugilism = 75,
    .Pistol = 60,
    .Ring = 50,
};

test "an item is created with its configured durability when the save holds nothing for it" {
    try std.testing.expectEqual(@as(i32, 300), initialDurability(300, 0));
}

test "an item already damaged in the save is created with what was left" {
    try std.testing.expectEqual(@as(i32, 100), initialDurability(300, 100));
}

test "an item without a configured durability is created with the game default" {
    try std.testing.expectEqual(default_durability, initialDurability(null, 0));
    try std.testing.expectEqual(default_durability, initialDurability(0, 0));
}

test "a sword breaks a 300 durability ore in three hits" {
    const damage = hitDamage(ore_weapon_damage, weaponKindFromRoleWeaponType(2));
    try std.testing.expectEqual(@as(i32, 100), damage);

    var durability: i32 = 300;
    durability = afterHit(durability, damage);
    try std.testing.expectEqual(@as(i32, 200), durability);
    durability = afterHit(durability, damage);
    try std.testing.expectEqual(@as(i32, 100), durability);
    durability = afterHit(durability, damage);
    try std.testing.expectEqual(@as(i32, 0), durability);
}

test "each weapon family removes the amount configured for it" {
    try std.testing.expectEqual(@as(i32, 150), hitDamage(ore_weapon_damage, weaponKindFromRoleWeaponType(1)));
    try std.testing.expectEqual(@as(i32, 60), hitDamage(ore_weapon_damage, weaponKindFromRoleWeaponType(3)));
    try std.testing.expectEqual(@as(i32, 75), hitDamage(ore_weapon_damage, weaponKindFromRoleWeaponType(4)));
    try std.testing.expectEqual(@as(i32, 50), hitDamage(ore_weapon_damage, weaponKindFromRoleWeaponType(5)));
}

test "a weapon family the item does not list falls back to the item default" {
    const only_default: ?TestWeaponDamage = .{ .DefaultValue = 40, .GreatSword = 80 };
    try std.testing.expectEqual(@as(i32, 40), hitDamage(only_default, .pistol));
    try std.testing.expectEqual(@as(i32, 40), hitDamage(only_default, .unknown));
}

test "an item without weapon damage configuration loses the game default per hit" {
    const none: ?TestWeaponDamage = null;
    const damage = hitDamage(none, .dagger);
    try std.testing.expectEqual(default_hit_damage, damage);
    // Items of durability 1 and 100 are both destroyed by a single hit.
    try std.testing.expectEqual(@as(i32, 0), afterHit(1, damage));
    try std.testing.expectEqual(@as(i32, 0), afterHit(100, damage));
}
