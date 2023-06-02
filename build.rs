// build script
use std::process::Command;
use std::fs::create_dir;
use std::path::Path;
use std::env;

fn main() {

    // Compile source
    Command::new("make").output()
        .expect("Failed to make");


    println!("cargo:rustc-link-search=c_interface");
    println!("cargo:rustc-link-search=fortran");
    println!("cargo:rustc-link-lib=static=pfapack");
    println!("cargo:rustc-link-lib=static=cpfapack");
    println!("cargo:rustc-link-lib=gfortran");
    println!("cargo:rustc-link-lib=lapack");
    println!("cargo:rustc-link-lib=blas");
}
