use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../build.zig");
    println!("cargo:rerun-if-changed=../backend/src/main.zig");
    println!("cargo:rerun-if-changed=../core/src/allocator.zig");
    
    // Get the Zig build output directory
    let out_dir = env::var("OUT_DIR").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_dir = PathBuf::from(&manifest_dir).parent().unwrap().to_path_buf();
    
    println!("Running Zig build from: {}", workspace_dir.display());
    
    // Build Zig backend first
    let zig_build_status = std::process::Command::new("zig")
        .arg("build")
        .current_dir(&workspace_dir)
        .status()
        .expect("Failed to execute zig build");
    
    if !zig_build_status.success() {
        panic!("Zig build failed");
    }
    
    // Tell cargo where to find the Zig libraries
    let zig_out = workspace_dir.join("zig-out/lib");
    println!("cargo:rustc-link-search=native={}", zig_out.display());
    
    // Link the Zig backend library
    println!("cargo:rustc-link-lib=static=ferrum_backend");
    println!("cargo:rustc-link-lib=static=ferrum_core");
    
    // Link system libraries that Zig backend needs
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=kernel32");
        println!("cargo:rustc-link-lib=user32");
    } else if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=c");
        println!("cargo:rustc-link-lib=m");
    } else if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=c");
        println!("cargo:rustc-link-lib=System");
    }
    
    // Generate Rust bindings for the Zig backend
    let header_path = workspace_dir.join("zig-out/ferrum_backend.h");
    
    if header_path.exists() {
        let bindings = bindgen::Builder::default()
            .header(header_path.to_str().unwrap())
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .generate()
            .expect("Unable to generate bindings");
        
        let out_path = PathBuf::from(&out_dir);
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    }
}
