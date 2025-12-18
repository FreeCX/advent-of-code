const std = @import("std");
const testing = std.testing;
const mem = std.mem;
const math = std.math;
const fmt = std.fmt;
const io = std.Io;
const util = @import("util.zig");

const Data = std.ArrayList([]u8);

fn parse(data: []u8, allocator: mem.Allocator) !Data {
    var reader = io.Reader.fixed(data);

    var result = try std.ArrayList([]u8).initCapacity(allocator, 200);
    while (true) {
        const raw = reader.takeDelimiterExclusive('\n');
        if (raw == error.EndOfStream) {
            break;
        }
        _ = try reader.discardShort(1);
        const line = try raw;
        if (line.len == 0) {
            break;
        }
        try result.append(allocator, try allocator.dupe(u8, line));
    }

    return result;
}

fn deinit(data: *Data, allocator: std.mem.Allocator) void {
    for (data.items) |item| {
        allocator.free(item);
    }
    defer data.deinit(allocator);
}

fn charToInt(char: u8) u8 {
    return char - 0x30;
}

fn findMaxOf(size: usize, data: []u8) u64 {
    if (size == 0) {
        return 0;
    }

    const max = mem.indexOfMax(u8, data[0 .. data.len - size + 1]);
    if (max == data.len - size) {
        return if (fmt.parseInt(u64, data[max .. max + size], 10)) |r| r else |_| unreachable;
    }

    const p = if (math.powi(u64, 10, size - 1)) |r| r else |_| unreachable;
    return charToInt(data[max]) * p + findMaxOf(size - 1, data[max + 1 ..]);
}

fn partOne(data: Data) u64 {
    var result: u64 = 0;
    for (data.items) |line| {
        result += findMaxOf(2, line);
    }
    return result;
}

fn partTwo(data: Data) u64 {
    var result: u64 = 0;
    for (data.items) |line| {
        result += findMaxOf(12, line);
    }
    return result;
}

pub fn main() !void {
    const buffer: [1024]u8 = undefined;

    const app = util.setup(@constCast(&buffer));
    defer app.deinit();

    const filename: []const u8 = "../data/day03.txt";
    const raw = try util.readFile(filename, app, @constCast(&buffer));
    defer app.allocator.free(raw);

    var parsed = try parse(raw, app.allocator);
    defer deinit(&parsed, app.allocator);

    const one = partOne(parsed);
    const two = partTwo(parsed);

    try app.stdout.print("part one: {d}\n", .{one});
    try app.stdout.print("part two: {d}\n", .{two});
    try app.stdout.flush();

    try testing.expectEqual(17263, one);
    try testing.expectEqual(170731717900423, two);
}

test "demo example" {
    const allocator = std.testing.allocator;
    const example = "987654321111111\n811111111111119\n234234234234278\n818181911112111\n";

    var parsed = try parse(@constCast(example), allocator);
    defer deinit(&parsed, allocator);

    try testing.expectEqual(357, partOne(parsed));
    try testing.expectEqual(3121910778619, partTwo(parsed));
}
