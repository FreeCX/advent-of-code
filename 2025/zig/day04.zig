const std = @import("std");
const testing = std.testing;
const mem = std.mem;
const io = std.Io;
const util = @import("util.zig");

const ItemType = enum { Empty, Roll };
const Data = struct {
    rows: u8,
    data: []ItemType,

    fn deinit(self: @This(), allocator: mem.Allocator) void {
        allocator.free(self.data);
    }
};

fn parse(data: []u8, allocator: mem.Allocator) !Data {
    var reader = io.Reader.fixed(data);

    const first = try reader.peekDelimiterExclusive('\n');
    const rows = first.len;
    var result = try allocator.alloc(ItemType, rows * rows);

    var row: usize = 0;
    while (reader.seek < data.len) {
        const line = try reader.takeDelimiterExclusive('\n');
        _ = try reader.take(1);
        for (line, 0..) |item, index| {
            result[row * rows + index] = if (item == '.') .Empty else .Roll;
        }
        row += 1;
    }

    return Data{ .rows = @intCast(rows), .data = result };
}

fn neighbors(data: Data, x: i16, y: i16) u8 {
    var count: u8 = 0;

    var yp = y - 1;
    while (yp <= y + 1) : (yp += 1) {
        if (yp < 0 or yp > data.rows - 1) continue;

        var xp = x - 1;
        while (xp <= x + 1) : (xp += 1) {
            if (xp < 0 or xp > data.rows - 1 or x == xp and y == yp) continue;

            const position = yp * data.rows + xp;
            if (data.data[@intCast(position)] == .Roll) {
                count += 1;
            }
        }
    }

    return count;
}

fn partOne(data: Data) u64 {
    var result: u64 = 0;

    for (0..data.rows) |y| {
        for (0..data.rows) |x| {
            const position = y * data.rows + x;
            if (data.data[position] == .Empty) continue;

            if (neighbors(data, @intCast(x), @intCast(y)) < 4) {
                result += 1;
            }
        }
    }

    return result;
}

fn partTwo(data: Data, allocator: mem.Allocator) !u64 {
    var result: u64 = 0;

    const size: u16 = @intCast(data.rows);
    var delete = try std.ArrayList(usize).initCapacity(allocator, size * size);
    defer delete.deinit(allocator);

    while (true) {
        for (0..data.rows) |y| {
            for (0..data.rows) |x| {
                const position = y * data.rows + x;
                if (data.data[position] == .Empty) continue;

                if (neighbors(data, @intCast(x), @intCast(y)) < 4) {
                    try delete.append(allocator, position);
                }
            }
        }

        if (delete.items.len == 0) break;

        result += delete.items.len;
        for (delete.items) |index| {
            data.data[index] = .Empty;
        }
        delete.clearRetainingCapacity();
    }

    return result;
}

pub fn main() !void {
    const buffer: [1024]u8 = undefined;

    const app = util.setup(@constCast(&buffer));
    defer app.deinit();

    const filename: []const u8 = "../data/day04.txt";
    const raw = try util.readFile(filename, app, @constCast(&buffer));
    defer app.allocator.free(raw);

    const parsed = try parse(raw, app.allocator);
    defer parsed.deinit(app.allocator);

    const one = partOne(parsed);
    const two = try partTwo(parsed, app.allocator);

    try app.stdout.print("part one: {d}\n", .{one});
    try app.stdout.print("part two: {d}\n", .{two});
    try app.stdout.flush();

    try testing.expectEqual(1356, one);
    try testing.expectEqual(8713, two);
}

test "demo example" {
    const allocator = std.testing.allocator;
    const example = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.\n";

    const parsed = try parse(@constCast(example), allocator);
    defer parsed.deinit(allocator);

    try testing.expectEqual(13, partOne(parsed));
    try testing.expectEqual(43, partTwo(parsed, allocator));
}
