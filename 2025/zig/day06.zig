const std = @import("std");
const testing = std.testing;
const mem = std.mem;
const fmt = std.fmt;
const io = std.Io;
const util = @import("util.zig");

const Operator = enum {
    PLUS,
    MULTIPLY,
};
const Data = struct {
    rows: usize,
    cols: usize,
    numbers: []u16,
    operators: []Operator,
    raw: [][]u8,

    fn deinit(self: @This(), allocator: mem.Allocator) void {
        allocator.free(self.operators);
        allocator.free(self.numbers);
        for (self.raw) |item| {
            allocator.free(item);
        }
        allocator.free(self.raw);
    }
};

fn parse(data: []u8, allocator: mem.Allocator) !Data {
    var reader = io.Reader.fixed(data);

    // count
    var rows: usize = 0;
    var cols: usize = 0;
    while (true) {
        const line = reader.takeDelimiterInclusive('\n');
        if (line == error.EndOfStream) {
            break;
        }
        if (cols == 0) {
            var iterator = mem.tokenizeAny(u8, try line, " \n");
            while (iterator.next()) |_| {
                cols += 1;
            }
        }
        rows += 1;
    }
    reader.seek = 0;

    // parse numbers
    var numbers = try allocator.alloc(u16, (rows - 1) * cols);
    var operators = try allocator.alloc(Operator, cols);
    var raw = try allocator.alloc([]u8, rows - 1);

    var n_index: usize = 0;
    for (0..rows - 1) |row| {
        const line = try reader.takeDelimiterInclusive('\n');
        raw[row] = try allocator.dupe(u8, line[0 .. line.len - 1]);

        var iterator = mem.tokenizeAny(u8, line, " \n");
        while (iterator.next()) |item| {
            const value = try fmt.parseInt(u16, item, 10);
            numbers[n_index] = value;
            n_index += 1;
        }
    }

    // parse operators
    const line = try reader.takeDelimiterInclusive('\n');
    var iterator = mem.tokenizeAny(u8, line, " \n");
    var o_index: usize = 0;
    while (iterator.next()) |item| {
        operators[o_index] = if (item[0] == '+') .PLUS else .MULTIPLY;
        o_index += 1;
    }

    return Data{
        .rows = rows,
        .cols = cols,
        .numbers = numbers,
        .operators = operators,
        .raw = raw,
    };
}

fn partOne(data: Data) u64 {
    var result: u64 = 0;

    for (0..data.cols) |col| {
        const operator = data.operators[col];
        var column_result: u64 = if (operator == .PLUS) 0 else 1;

        for (0..data.rows - 1) |row| {
            const index = data.cols * row + col;
            if (operator == .PLUS) {
                column_result += data.numbers[index];
            } else {
                column_result *= data.numbers[index];
            }
        }

        result += column_result;
    }

    return result;
}

fn findSeparators(data: Data, allocator: mem.Allocator) ![]u16 {
    var separators = try allocator.alloc(u16, data.cols + 1);
    var index: u16 = 1;

    separators[0] = 0;
    outer: for (0..data.raw[0].len) |pos| {
        for (data.raw) |item| {
            if (item[pos] != ' ') {
                continue :outer;
            }
        }
        separators[index] = @intCast(pos);
        index += 1;
    }
    separators[data.cols] = @intCast(data.raw[0].len);

    return separators;
}

fn partTwo(data: Data, allocator: mem.Allocator) !u64 {
    var result: u64 = 0;

    const separators = try findSeparators(data, allocator);
    defer allocator.free(separators);

    var col: u16 = 0;
    while (col < separators.len - 1) {
        const left = separators[col];
        const right = separators[col + 1];
        const operator = data.operators[col];

        var column_result: u64 = if (operator == .PLUS) 0 else 1;
        for (left..right) |pos| {
            var number: u64 = 0;
            for (0..data.rows - 1) |row| {
                const item = data.raw[row][pos];
                if (item != ' ') {
                    number = number * 10 + (item - 0x30);
                }
            }
            if (number == 0) continue;

            if (operator == .PLUS) {
                column_result += number;
            } else {
                column_result *= number;
            }
        }

        result += column_result;
        col += 1;
    }

    return result;
}

pub fn main() !void {
    const buffer: [1024]u8 = undefined;

    const app = util.setup(@constCast(&buffer));
    defer app.deinit();

    const filename: []const u8 = "../data/day06.txt";
    const raw = try util.readFile(filename, app, @constCast(&buffer));
    defer app.allocator.free(raw);

    const parsed = try parse(raw, app.allocator);
    defer parsed.deinit(app.allocator);

    const one = partOne(parsed);
    const two = try partTwo(parsed, app.allocator);

    try app.stdout.print("part one: {d}\n", .{one});
    try app.stdout.print("part two: {d}\n", .{two});
    try app.stdout.flush();

    try testing.expectEqual(5784380717354, one);
    try testing.expectEqual(7996218225744, two);
}

test "demo example" {
    const allocator = std.testing.allocator;
    const example = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  \n";

    const parsed = try parse(@constCast(example), allocator);
    defer parsed.deinit(allocator);

    const numbers = [_]u16{ 123, 328, 51, 64, 45, 64, 387, 23, 6, 98, 215, 314 };
    try testing.expect(mem.eql(u16, &numbers, parsed.numbers));
    const operators = [_]Operator{ .MULTIPLY, .PLUS, .MULTIPLY, .PLUS };
    try testing.expect(mem.eql(Operator, &operators, parsed.operators));
    try testing.expect(mem.eql(u8, example[0..15], parsed.raw[0]));

    try testing.expectEqual(4277556, partOne(parsed));
    try testing.expectEqual(3263827, try partTwo(parsed, allocator));
}
