use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    // Build voor de host architectuur
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
    
    // Kopieer bestanden naar de ISO build directory
    let out_dir = env::var("OUT_DIR").unwrap();
    let iso_dir = Path::new(&out_dir).join("iso");
    
    Command::new("cp")
        .args(&[
            "target/release/arch-installer",
            &format!("{}/airootfs/root/", iso_dir.display())
        ])
        .status()
        .expect("Failed to copy installer");
        
    Command::new("cp")
        .args(&[
            "installer.service",
            &format!("{}/airootfs/root/", iso_dir.display())
        ])
        .status()
        .expect("Failed to copy service");
} 