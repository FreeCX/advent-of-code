const std = @import("std");

pub fn setup(buffer: []u8) type {
    return struct {
        var stdout_writer = std.fs.File.stdout().writer(buffer);
        var stderr_writer = std.fs.File.stderr().writer(buffer);
        var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);

        pub const allocator = arena.allocator();
        pub const stdout = &stdout_writer.interface;
        pub const stderr = &stderr_writer.interface;

        pub fn deinit() void {
            _ = arena.deinit();
        }
    };
}

pub fn readFile(filename: []const u8, app: type, buffer: []u8) ![]u8 {
    const file = try std.fs.cwd().openFile(filename, .{});
    defer file.close();

    var reader = file.reader(buffer);
    const size = try reader.getSize();
    const data = try app.allocator.alloc(u8, size);
    try reader.interface.readSliceAll(data);

    return data;
}
