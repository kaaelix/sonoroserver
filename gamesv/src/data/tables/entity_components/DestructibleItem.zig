const std = @import("std");

/// Durability removed by one hit, per weapon family of the attacker.
pub const WeaponDamageConfig = struct {
    DefaultValue: ?i32 = null,
    GreatSword: ?i32 = null,
    Dagger: ?i32 = null,
    Pugilism: ?i32 = null,
    Pistol: ?i32 = null,
    Ring: ?i32 = null,
};

Disabled: ?bool = null,
Durability: ?i32 = null,
DurabilityId: ?i32 = null,
WeaponDamage: ?WeaponDamageConfig = null,
DurabilityStateConfig: std.json.Value = .null,
DestructionActions: std.json.Value = .null,
