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

    // Get the target architecture from Cargo
    let target = env::var("TARGET").unwrap_or_else(|_| "native".to_string());
    println!("Building for target: {target}");

    // Convert Rust target to Zig target format
    let zig_target = match target.as_str() {
        "x86_64-apple-darwin" => "x86_64-macos",
        "aarch64-apple-darwin" => "aarch64-macos",
        "x86_64-unknown-linux-gnu" => "x86_64-linux",
        "aarch64-unknown-linux-gnu" => "aarch64-linux",
        "x86_64-pc-windows-msvc" => "x86_64-windows",
        _ => "native", // Fallback to native compilation
    };

    // Build Zig backend with explicit target
    let mut zig_cmd = std::process::Command::new("zig");
    zig_cmd.arg("build").current_dir(&workspace_dir);

    if zig_target != "native" {
        zig_cmd.arg(format!("-Dtarget={zig_target}"));
    }

    let zig_build_status = zig_cmd.status().expect("Failed to execute zig build");

    if !zig_build_status.success() {
        panic!("Zig build failed for target: {zig_target}" );
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
