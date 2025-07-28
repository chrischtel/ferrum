// C wrapper functions for FFI
// This file bridges the Zig backend with Rust frontend

void* ferrum_create_context(void);
void ferrum_destroy_context(void* ctx);
void* ferrum_create_module(void* ctx, const char* name);
int ferrum_compile_to_object(void* module, const char* output_path);
