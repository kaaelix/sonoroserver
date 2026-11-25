//! Test root for gamesv: `build.zig` has no test step, and `zig test` only collects the tests of
//! files referenced from a test, so the files that carry tests are listed here.
//!
//! From the repository root (inside WSL, like the build):
//!   zig test /lib/x86_64-linux-gnu/libsqlite3.so.0 --dep common --dep proto \
//!     -Mroot=gamesv/src/tests.zig -Mcommon=common/src/root.zig -Mproto=proto/src/root.zig -lc

test {
    _ = @import("logic/component/entity/FsmComponent.zig");
    _ = @import("logic/helpers/destructible.zig");
    _ = @import("logic/helpers/save_block.zig");
    _ = @import("logic/helpers/quest_engine.zig");
    _ = @import("network/handlers/buff.zig");
}
