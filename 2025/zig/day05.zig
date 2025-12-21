const std = @import("std");
const testing = std.testing;
const mem = std.mem;
const math = std.math;
const fmt = std.fmt;
const io = std.Io;
const util = @import("util.zig");

const Range = struct {
    start: u64,
    stop: u64,

    fn point_in(self: @This(), point: u64) bool {
        return point >= self.start and point <= self.stop;
    }

    fn intersect(self: @This(), other: @This()) bool {
        return self.point_in(other.start) or self.point_in(other.stop) or
            other.point_in(self.start) or other.point_in(self.stop);
    }

    fn merge(self: @This(), other: @This()) @This() {
        return Range{ .start = @min(self.start, other.start), .stop = @max(self.stop, other.stop) };
    }

    fn compare(_: void, lhs: Range, rhs: Range) bool {
        return lhs.start < rhs.start;
    }
};

const Data = struct {
    ranges: []Range,
    ingredients: []u64,

    fn deinit(self: @This(), allocator: mem.Allocator) void {
        allocator.free(self.ranges);
        allocator.free(self.ingredients);
    }
};

fn parse(data: []u8, allocator: mem.Allocator) !Data {
    var reader = io.Reader.fixed(data);

    var ranges = try std.ArrayList(Range).initCapacity(allocator, 64);
    defer ranges.deinit(allocator);

    while (true) {
        const line = try reader.takeDelimiterExclusive('\n');
        const separator = mem.indexOf(u8, line, "-");

        const start = try fmt.parseInt(u64, line[0..separator.?], 10);
        const stop = try fmt.parseInt(u64, line[separator.? + 1 ..], 10);
        try ranges.append(allocator, Range{ .start = start, .stop = stop });

        if (mem.eql(u8, try reader.peek(2), "\n\n")) break;
        _ = try reader.take(1);
    }

    _ = try reader.take(2);

    var ingredients = try std.ArrayList(u64).initCapacity(allocator, 64);
    defer ingredients.deinit(allocator);

    while (true) {
        const raw = reader.takeDelimiterExclusive('\n');
        if (raw == error.EndOfStream) {
            break;
        }

        _ = try reader.take(1);
        const line = try raw;

        const ingredient = try fmt.parseInt(u64, line, 10);
        try ingredients.append(allocator, ingredient);
    }

    return Data{
        .ranges = try allocator.dupe(Range, ranges.items),
        .ingredients = try allocator.dupe(u64, ingredients.items),
    };
}

fn partOne(data: Data) u64 {
    var result: u64 = 0;

    for (data.ingredients) |ingredient| {
        for (data.ranges) |range| {
            if (range.point_in(ingredient)) {
                result += 1;
                break;
            }
        }
    }

    return result;
}

fn partTwo(data: Data, allocator: mem.Allocator) !u64 {
    var temp = try std.ArrayList(Range).initCapacity(allocator, data.ranges.len);
    defer temp.deinit(allocator);

    for (data.ranges) |a| {
        var current = a;

        for (data.ranges[1..]) |b| {
            if (current.intersect(b)) {
                current = current.merge(b);
            }
        }

        var have_intersect = false;
        for (0..temp.items.len) |index| {
            const r = temp.items[index];
            if (current.intersect(r)) {
                temp.items[index] = current.merge(r);
                have_intersect = true;
                break;
            }
        }

        if (!have_intersect) {
            try temp.append(allocator, current);
        }
    }

    var result: u64 = 0;
    for (temp.items) |range| {
        result += (range.stop - range.start + 1);
    }

    return result;
}

pub fn main() !void {
    const buffer: [1024]u8 = undefined;

    const app = util.setup(@constCast(&buffer));
    defer app.deinit();

    const filename: []const u8 = "../data/day05.txt";
    const raw = try util.readFile(filename, app, @constCast(&buffer));
    defer app.allocator.free(raw);

    const parsed = try parse(raw, app.allocator);
    defer parsed.deinit(app.allocator);

    // preprocess ranges
    mem.sort(Range, parsed.ranges, {}, Range.compare);

    const one = partOne(parsed);
    const two = try partTwo(parsed, app.allocator);

    try app.stdout.print("part one: {d}\n", .{one});
    try app.stdout.print("part two: {d}\n", .{two});
    try app.stdout.flush();

    try testing.expectEqual(513, one);
    try testing.expectEqual(339668510830757, two);
}

test "demo example" {
    const allocator = std.testing.allocator;
    const example = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32\n";

    const parsed = try parse(@constCast(example), allocator);
    defer parsed.deinit(allocator);

    try testing.expectEqual(3, partOne(parsed));
    try testing.expectEqual(14, try partTwo(parsed, allocator));
}
