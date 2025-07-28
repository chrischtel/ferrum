const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const core = b.addStaticLibrary(.{
        .name = "ferrum_core",
        .root_source_file = b.path("src/allocator.zig"),
        .target = target,
        .optimize = optimize,
    });

    core.linkLibC();
    b.installArtifact(core);
}
