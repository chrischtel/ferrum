use clap::{Arg, Command};
use ferrum_frontend::*;
use std::ffi::CString;

fn main() -> anyhow::Result<()> {
    let matches = Command::new("ferrum")
        .version("0.1.0")
        .about("Ferrum Programming Language Compiler")
        .arg(
            Arg::new("input")
                .help("Input source file")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output file")
                .default_value("output.o"),
        )
        .get_matches();

    let input_file = matches.get_one::<String>("input").unwrap();
    let output_file = matches.get_one::<String>("output").unwrap();

    println!("Compiling {} to {}", input_file, output_file);

    // Test backend integration
    unsafe {
        let ctx = ferrum_create_context();
        if ctx.is_null() {
            anyhow::bail!("Failed to create compiler context");
        }

        let module_name = CString::new("test_module")?;
        let module = ferrum_create_module(ctx, module_name.as_ptr());
        
        if !module.is_null() {
            let output_path = CString::new(output_file.as_str())?;
            let result = ferrum_compile_to_object(module, output_path.as_ptr());
            
            if result == 0 {
                println!("Compilation successful!");
            } else {
                println!("Compilation failed with code: {}", result);
            }
        }

        ferrum_destroy_context(ctx);
    }

    Ok(())
}
