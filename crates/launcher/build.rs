use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=../../Cargo.lock");
    println!("cargo:rerun-if-changed=../../Cargo.toml");
    println!("cargo:rerun-if-changed=../core");
    println!("cargo:rerun-if-changed=../../target/debug/libv8_killer_core.so");
    println!("cargo:rerun-if-changed=../../target/debug/libv8_killer_core.dll");
    println!("cargo:rerun-if-changed=../../target/debug/libv8_killer_core.dylib");
    println!("cargo:rerun-if-changed=../../target/release/libv8_killer_core.so");
    println!("cargo:rerun-if-changed=../../target/release/libv8_killer_core.dll");
    println!("cargo:rerun-if-changed=../../target/release/libv8_killer_core.dylib");

    // 判断当前平台
    let target = env::var("TARGET").unwrap();
    let profile = env::var("PROFILE").unwrap();

    let (filename, filepath) = match target {
        t if t.contains("linux") => {
            ("libv8_killer_core.so", format!("../../target/{}/libv8_killer_core.so", profile))
        }
        t if t.contains("windows") => {
            ("libv8_killer_core.dll", format!("../../target/{}/v8_killer_core.dll", profile))
        }
        t if t.contains("darwin") => {
            ("libv8_killer_core.dylib", format!("../../target/{}/libv8_killer_core.dylib", profile))
        }
        t => {
            panic!("unsupported target: {}", t);
        }
    };
    println!("cargo:rerun-if-changed={}", filepath);

    let out_dir_str = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir_str);
    let mut lib_file = File::create(
        out_dir.join("LIB_FILE")
    ).unwrap();

    let mut filename_file = File::create(
        out_dir.join("LIB_FILENAME")
    ).unwrap();

    lib_file.write_all(
        &std::fs::read(filepath).unwrap()
    ).unwrap();
    filename_file.write_all(filename.as_bytes()).unwrap();
}