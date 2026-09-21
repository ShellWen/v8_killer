fn main() {
    println!("cargo:rerun-if-changed=native");
    println!("cargo:rerun-if-env-changed=V8_KILLER_DOBBY_ARCHIVE");
    let dst = cmake::Config::new("native").profile("Release").build();
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=v8_killer_native");
    println!("cargo:rustc-link-lib=static=dobby");
    let target = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target == "macos" {
        println!("cargo:rustc-link-lib=c++");
    } else if std::env::var("CARGO_CFG_TARGET_ENV").unwrap() != "msvc" {
        println!("cargo:rustc-link-lib=stdc++");
    }
    if target == "linux" {
        println!("cargo:rustc-link-lib=dl");
    }
    if target == "windows" {
        println!("cargo:rustc-link-lib=psapi");
    }
}
