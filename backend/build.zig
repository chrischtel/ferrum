const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const backend = b.addStaticLibrary(.{
        .name = "ferrum_backend",
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    // TODO: Add LLVM linking later
    // backend.linkSystemLibrary("LLVM");
    backend.linkLibC();

    b.installArtifact(backend);
}
