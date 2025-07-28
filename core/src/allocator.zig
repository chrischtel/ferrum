const std = @import("std");

// Simple allocator wrapper for Ferrum core runtime
var gpa = std.heap.GeneralPurposeAllocator(.{}){};

export fn ferrum_alloc(size: usize) ?*anyopaque {
    const allocator = gpa.allocator();
    const mem = allocator.alloc(u8, size) catch return null;
    return mem.ptr;
}

export fn ferrum_free(ptr: ?*anyopaque, size: usize) void {
    if (ptr) |p| {
        const allocator = gpa.allocator();
        const slice = @as([*]u8, @ptrCast(p))[0..size];
        allocator.free(slice);
    }
}

export fn ferrum_realloc(ptr: ?*anyopaque, old_size: usize, new_size: usize) ?*anyopaque {
    if (ptr == null) {
        return ferrum_alloc(new_size);
    }

    const allocator = gpa.allocator();
    const old_slice = @as([*]u8, @ptrCast(ptr))[0..old_size];
    const new_slice = allocator.realloc(old_slice, new_size) catch return null;
    return new_slice.ptr;
}
