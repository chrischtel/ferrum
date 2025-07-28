// Zig build configuration for Ferrum
const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Backend library
    const backend = b.addStaticLibrary(.{
        .name = "ferrum_backend",
        .root_source_file = b.path("backend/src/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    // Core runtime library
    const core = b.addStaticLibrary(.{
        .name = "ferrum_core",
        .root_source_file = b.path("core/src/allocator.zig"),
        .target = target,
        .optimize = optimize,
    });

    // TODO: Add LLVM linking later
    // backend.linkSystemLibrary("LLVM");
    backend.linkLibC();

    // Install artifacts
    b.installArtifact(backend);
    b.installArtifact(core);

    // Create header file for Rust FFI
    const header_step = b.addWriteFiles();
    _ = header_step.add("ferrum_backend.h",
        \\#ifndef FERRUM_BACKEND_H
        \\#define FERRUM_BACKEND_H
        \\
        \\#include <stdint.h>
        \\#include <stddef.h>
        \\
        \\#ifdef __cplusplus
        \\extern "C" {
        \\#endif
        \\
        \\// Opaque types
        \\typedef struct Ferrum_Context Ferrum_Context;
        \\typedef struct Ferrum_Module Ferrum_Module;
        \\
        \\// Core functions
        \\Ferrum_Context* ferrum_create_context(void);
        \\void ferrum_destroy_context(Ferrum_Context* ctx);
        \\Ferrum_Module* ferrum_create_module(Ferrum_Context* ctx, const char* name);
        \\int ferrum_compile_to_object(Ferrum_Module* module, const char* output_path);
        \\
        \\#ifdef __cplusplus
        \\}
        \\#endif
        \\
        \\#endif // FERRUM_BACKEND_H
    );

    b.getInstallStep().dependOn(&header_step.step);
}
