// rust_bindgen_recursive.rs

use std::process::Command;
use std::fs;
use std::path::Path;

fn main() {
    println!("Engaging in the most Rust-ian adventure of binding C to Rust. Buckle up!");

    // Assuming your git submodule is named 'flipperzero-firmware'
    let submodule_path = Path::new("flipperzero-firmware");
    if !submodule_path.exists() {
        println!("No submodule found! Did you forget to git submodule update --init?");
        return;
    }

    // Recursively find all .h files
    for entry in walkdir::WalkDir::new(submodule_path).into_iter().filter_map(Result::ok) {
        if let Some("h") = entry.path().extension().and_then(|s| s.to_str()) {
            let header = entry.path();
            let bindings_path = header.with_extension("rs");

            // Generate bindings
            println!("Generating bindings for {:?}", header);
            Command::new("bindgen")
                .arg(header)
                .arg("--output")
                .arg(bindings_path)
                .status()
                .expect("Failed to run bindgen");

            println!("Bindings generated at {:?}", bindings_path);
        }
    }

    println!("Operation complete. You now have Rust bindings for all the C you can handle!");
}