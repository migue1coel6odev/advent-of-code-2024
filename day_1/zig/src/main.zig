const std = @import("std");
const input = @embedFile("input.txt");

pub fn main() !void {
    var splits = std.mem.split(u8, input, "\n");

    var first_list = @Vector(splits.buffer.len, u32);
    var second_list = @Vector(splits.buffer.len, u32);

    while (splits.next()) |line| {
        var splittedLine = std.mem.split(u8, line, "   ");

        var first: ?u32 = null;
        if (splittedLine.next()) |first_value| {
            first = try std.fmt.parseInt(u32, first_value, 10);
            first_list
                .std.debug.print("First String: {?s} | First Number: {d}\n", .{ first_value, first });
        }

        var second: ?u32 = null;
        if (splittedLine.next()) |second_value| {
            second = try std.fmt.parseInt(u32, second_value, 10);
        }
    }
}
