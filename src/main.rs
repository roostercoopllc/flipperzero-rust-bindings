// rust_bindgen_flipperzero_mirror.rs

use std::process::Command;
use std::fs;
use std::path::Path;

fn main() {
    println!("Mirroring Flipper Zero's firmware in Rust. This is going to be epic!");

    // Path to the Flipper Zero firmware submodule
    let submodule_path = Path::new("flipperzero-firmware");
    if !submodule_path.exists() {
        println!("No Flipper Zero firmware found! Did you forget to git submodule update --init?");
        return;
    }

    // Path where we'll output the Rust bindings
    let output_path = Path::new("flipperzero-firmware-rust");
    if !output_path.exists() {
        fs::create_dir_all(output_path).expect("Failed to create output directory");
    }

    // Recursively find all .h files in the firmware directory
    for entry in walkdir::WalkDir::new(submodule_path).into_iter().filter_map(Result::ok) {
        if let Some("h") = entry.path().extension().and_then(|s| s.to_str()) {
            let header = entry.path();
            let relative_path = header.strip_prefix(submodule_path).unwrap();
            let output_file = output_path.join(relative_path).with_extension("rs");

            // Ensure the directory structure exists in the output path
            if let Some(parent) = output_file.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent).expect("Failed to create directory structure");
                }
            }

            // Generate bindings
            println!("Generating Rust bindings for {:?}", header);
            Command::new("bindgen")
                .arg(header)
                .arg("--output")
                .arg(output_file.clone())
                .status()
                .expect("Failed to run bindgen. Did it get stuck in a firmware loop?");

            println!("Bindings for Flipper Zero generated at {:?}", output_file);
        }
    }

    println!("Operation complete! You've got a Rust mirror of Flipper Zero's firmware. Now, go flip the world!");
}