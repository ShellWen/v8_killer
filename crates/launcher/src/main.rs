use std::env::current_exe;
use v8_killer_launcher::{default_lib_filename, launch};

fn main() {
    // TODO: Support custom lib_filename
    let lib_filename = default_lib_filename().unwrap();
    let mut lib_path = current_exe().unwrap().parent().unwrap().to_path_buf();
    lib_path.push(lib_filename);
    let lib_path_str = lib_path.to_str().unwrap();

    let exe_cmdline = std::env::args().nth(1).expect("no exe_cmdline provided");

    println!("[*] Executable cmdline: {}", exe_cmdline);
    println!("[*] Core lib path: {}", lib_path_str);
    launch(lib_path_str, &exe_cmdline);
}
