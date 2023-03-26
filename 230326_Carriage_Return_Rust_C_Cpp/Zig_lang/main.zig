const std = @import("std");
const ns_per_us: u64 = 1000;
const ns_per_ms: u64 = 1000 * ns_per_us;
const ns_per_s: u64 = 1000 * ns_per_ms;

pub fn main() void {
    const chars = [_]u8{ '-', '\\', '|', '/' };
    var i: u32 = 0;
    var msec: u32 = 200;

    while (true) {
        std.debug.print("{c}\r", .{chars[i % chars.len]});
        std.time.sleep(msec * ns_per_ms);
        i += 1;
    }
}
