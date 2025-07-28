const std = @import("std");

// Temporary backend implementation
// TODO: Add LLVM integration later

export fn ferrum_create_context() ?*anyopaque {
    return null; // Placeholder
}

export fn ferrum_destroy_context(ctx: ?*anyopaque) void {
    _ = ctx; // Placeholder
}

export fn ferrum_create_module(ctx: ?*anyopaque, name: [*:0]const u8) ?*anyopaque {
    _ = ctx;
    _ = name;
    return null; // Placeholder
}

export fn ferrum_compile_to_object(module: ?*anyopaque, output_path: [*:0]const u8) i32 {
    _ = module;
    _ = output_path;
    return 0; // Placeholder success
}
