Id: i32,
StateMachine: []const u8 = "",
SubBehaviorConfigs: ?struct {
    AiHate: ?[]const u8 = null,
    AiSense: ?[]const u8 = null,
    AiAlert: ?[]const u8 = null,
    AiPatrol: ?[]const u8 = null,
} = null,
