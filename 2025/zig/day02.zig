const std = @import("std");
const testing = std.testing;
const math = std.math;
const mem = std.mem;
const io = std.Io;
const util = @import("util.zig");

const Data = struct { first: u64, last: u64 };

fn parse(data: []u8, allocator: mem.Allocator) ![]Data {
    var reader = io.Reader.fixed(data);

    var result = try std.ArrayList(Data).initCapacity(allocator, 64);
    defer result.deinit(allocator);

    while (true) {
        const first_part = reader.takeDelimiterInclusive('-');
        if (first_part == error.EndOfStream) {
            break;
        }
        const first = try first_part;

        var second_part = reader.takeDelimiterInclusive(',');
        if (second_part == error.EndOfStream) {
            second_part = reader.takeDelimiterInclusive('\n');
        }
        const second = try second_part;

        const first_id = try std.fmt.parseInt(u64, first[0 .. first.len - 1], 10);
        const last_id = try std.fmt.parseInt(u64, second[0 .. second.len - 1], 10);

        try result.append(allocator, .{ .first = first_id, .last = last_id });
    }

    return try allocator.dupe(Data, result.items);
}

fn isRepeatTwice(value: u64) bool {
    const e: u64 = math.log10_int(value) + 1;
    if (e % 2 != 0) {
        return false;
    }
    const p = math.pow(u64, 10, e / 2);
    const high: u64 = @divTrunc(value, p);
    const low: u64 = @mod(value, p);
    return high == low;
}

fn partOne(data: []Data) u64 {
    var result: u64 = 0;
    for (data) |item| {
        for (item.first..item.last + 1) |id| {
            if (isRepeatTwice(id)) {
                result += id;
            }
        }
    }
    return result;
}

// TODO: это не эффективно, но работает
fn isRepeatAny(value: u64) bool {
    var buffer: [10]u8 = undefined;
    const text = if (std.fmt.bufPrint(&buffer, "{d}", .{value})) |r| r else |_| unreachable;

    for (1..text.len / 2 + 1) |length| {
        if (@mod(text.len, length) != 0) {
            continue;
        }

        const substring = text[0..length];
        const repeat_count = @divTrunc(text.len, length);
        var count: u8 = 0;
        for (0..repeat_count) |index| {
            const pos = index * length;
            if (std.mem.eql(u8, text[pos .. pos + length], substring)) {
                count += 1;
            }
        }
        if (count == repeat_count) {
            return true;
        }
    }

    return false;
}

fn partTwo(data: []Data) u64 {
    var result: u64 = 0;
    for (data) |item| {
        for (item.first..item.last + 1) |id| {
            if (isRepeatAny(id)) {
                result += id;
            }
        }
    }
    return result;
}

pub fn main() !void {
    const buffer: [1024]u8 = undefined;

    const app = util.setup(@constCast(&buffer));
    defer app.deinit();

    const filename: []const u8 = "../data/day02.txt";
    const raw = try util.readFile(filename, app, @constCast(&buffer));
    defer app.allocator.free(raw);

    const parsed = try parse(raw, app.allocator);
    defer app.allocator.free(parsed);

    const one = partOne(parsed);
    const two = partTwo(parsed);

    try app.stdout.print("part one: {d}\n", .{one});
    try app.stdout.print("part two: {d}\n", .{two});
    try app.stdout.flush();

    try testing.expectEqual(53420042388, one);
    try testing.expectEqual(69553832684, two);
}

test "demo example" {
    const allocator = std.testing.allocator;
    const example = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124\n";

    const parsed = try parse(@constCast(example), allocator);
    defer allocator.free(parsed);

    try testing.expectEqual(1227775554, partOne(parsed));
    try testing.expectEqual(4174379265, partTwo(parsed));
}
