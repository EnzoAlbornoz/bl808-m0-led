// Import Dependencies
use std::path::Path;
// Define Build Script
fn main() {
    // Define linker script path
    let linker_script = Path::new("link").join("memory").with_extension("x");
    let linker_location_string = linker_script
        .canonicalize()
        .ok()
        .and_then(|path| Some(path.to_str()?.to_string()))
        .unwrap();
    // Apply linkers
    println!("cargo:rustc-link-arg-bins=-T{}", &linker_location_string);
    println!("cargo:rustc-link-arg-bins=-T{}", "link.x");
    // Set Watchers
    println!("cargo:rerun-if-changed={}", &linker_location_string);
    println!("cargo:rerun-if-changed=build.rs");
}
