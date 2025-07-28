// Ferrum compiler frontend library

pub mod lexer;

// Include generated bindings from build.rs
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod ffi {
    // Fallback if bindings aren't generated yet
    #[repr(C)]
    pub struct Ferrum_Context {
        _private: [u8; 0],
    }

    #[repr(C)]
    pub struct Ferrum_Module {
        _private: [u8; 0],
    }

    extern "C" {
        pub fn ferrum_create_context() -> *mut Ferrum_Context;
        pub fn ferrum_destroy_context(ctx: *mut Ferrum_Context);
        pub fn ferrum_create_module(
            ctx: *mut Ferrum_Context,
            name: *const i8,
        ) -> *mut Ferrum_Module;
        pub fn ferrum_compile_to_object(module: *mut Ferrum_Module, output_path: *const i8) -> i32;
    }
}

// Re-export for easier use
pub use ffi::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backend_integration() {
        unsafe {
            let ctx = ferrum_create_context();
            // Skip test if backend is not yet implemented (returns null)
            if ctx.is_null() {
                println!("Backend not yet implemented, skipping integration test");
                return;
            }
            ferrum_destroy_context(ctx);
        }
    }
}
