// build script
use std::process::Command;
use std::fs::create_dir_all;
use std::path::Path;
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);
    let c_interface = out_dir.join(Path::new("c_interface"));
    let fortran = out_dir.join(Path::new("fortran"));

    // Create the c_interface directory
    create_dir_all(c_interface.clone())
        .expect("Can't create c_interface directory");
    // Create the fortran directory
    create_dir_all(fortran.clone())
        .expect("Can't create fortran directory");

    // Compile source
    Command::new("make").output()
        .expect("Failed to make");


    println!("cargo:rustc-link-search={}", c_interface.display());
    println!("cargo:rustc-link-search={}", fortran.display());
    println!("cargo:rustc-link-lib=gfortran");
    println!("cargo:rustc-link-lib=lapack");
    println!("cargo:rustc-link-lib=blas");
}
