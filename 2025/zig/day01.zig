const std = @import("std");
const testing = std.testing;
const mem = std.mem;
const io = std.Io;
const util = @import("util.zig");

const DirectionType = enum { Left, Right };
const Data = struct { direction: DirectionType, angle: i16 };

fn parse(data: []u8, allocator: mem.Allocator) ![]Data {
    var reader = io.Reader.fixed(data);

    var result = try std.ArrayList(Data).initCapacity(allocator, 4110);
    defer result.deinit(allocator);

    while (true) {
        const raw = reader.takeDelimiterInclusive('\n');
        if (raw == error.EndOfStream) {
            break;
        }
        const line = try raw;
        const direction: DirectionType = if (line[0] == 'R') .Right else .Left;
        const angle = try std.fmt.parseInt(i16, line[1 .. line.len - 1], 10);

        try result.append(allocator, .{ .direction = direction, .angle = angle });
    }

    return try allocator.dupe(Data, result.items);
}

fn partOne(data: []Data) u16 {
    var count: u16 = 0;
    var current: i16 = 50;
    for (data) |item| {
        const dir: i16 = if (item.direction == .Left) -1 else 1;
        current = @mod(current + item.angle * dir, 100);
        if (current == 0) {
            count += 1;
        }
    }
    return count;
}

// TODO: избавиться от внутреннего цикла
fn partTwo(data: []Data) i16 {
    var count: i16 = 0;
    var current: i16 = 50;
    for (data) |item| {
        const dir: i16 = if (item.direction == .Left) -1 else 1;
        for (0..@intCast(item.angle)) |_| {
            current += dir;
            if (@mod(current, 100) == 0) {
                count += 1;
            }
        }
    }
    return count;
}

pub fn main() !void {
    const buffer: [1024]u8 = undefined;

    const app = util.setup(@constCast(&buffer));
    defer app.deinit();

    const filename: []const u8 = "../data/day01.txt";
    const raw = try util.readFile(filename, app, @constCast(&buffer));
    defer app.allocator.free(raw);

    const parsed = try parse(raw, app.allocator);
    defer app.allocator.free(parsed);

    const one = partOne(parsed);
    const two = partTwo(parsed);

    try app.stdout.print("part one: {d}\n", .{one});
    try app.stdout.print("part two: {d}\n", .{two});
    try app.stdout.flush();

    try testing.expectEqual(1043, one);
    try testing.expectEqual(5963, two);
}

test "demo example" {
    const allocator = std.testing.allocator;
    const example = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82\n";

    const parsed = try parse(@constCast(example), allocator);
    defer allocator.free(parsed);

    try testing.expectEqual(3, partOne(parsed));
    try testing.expectEqual(6, partTwo(parsed));
}

test "reddit example" {
    const allocator = std.testing.allocator;
    const example = "R1000\nL1000\nL50\nR1\nL1\nL1\nR1\nR100\nR1\n";

    const parsed = try parse(@constCast(example), allocator);
    defer allocator.free(parsed);

    try testing.expectEqual(24, partTwo(parsed));
}
