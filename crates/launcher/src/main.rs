use std::env::current_exe;

use v8_killer_launcher::{default_lib_filename, launch};

fn main() {
    // TODO: Support custom lib_filename
    let lib_filename = default_lib_filename().unwrap();
    let mut lib_path = current_exe().unwrap().parent().unwrap().to_path_buf();
    lib_path.push(lib_filename);
    let lib_path_str = lib_path.to_str().unwrap();

    let exe = std::env::args().nth(1).expect("no executable provided");
    let args = std::env::args().skip(2).collect::<Vec<String>>();

    println!("[*] Executable: {}", exe);
    println!("[*] Args: {:?}", args);
    println!("[*] Core lib path: {}", lib_path_str);
    launch(lib_path_str, &exe, &args);
}
