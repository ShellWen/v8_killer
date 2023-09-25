use std::process::{ExitStatus, Stdio};

const LIB_FILENAME: &'static str =
    include_str!(concat!(env!("OUT_DIR"), "/LIB_FILENAME"));
const LIB_FILE: &'static [u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/LIB_FILE"));

#[cfg(target_os = "linux")]
fn launch(lib_path: &str, exe_cmdline: &str) {
    use std::process::Command;

    println!("exe_cmdline: {}", exe_cmdline);

    // 分割 cmdline 中的文件路径和参数
    // 需要考虑路径可能包含空格的情况
    let (exe_path, args) = if exe_cmdline.starts_with("\"") {
        let mut iter = exe_cmdline[1..].split("\"");
        let exe_path = iter.next().unwrap();
        let args = iter.next().unwrap_or("").trim();
        (exe_path, args)
    } else {
        let mut iter = exe_cmdline.split(" ");
        let exe_path = iter.next().unwrap();
        let args = iter.next().unwrap_or("").trim();
        (exe_path, args)
    };

    let args_vec: Vec<&str> = args.split(" ").collect();

    let mut child = Command::new(exe_path)
        .args(args_vec)
        .env("LD_PRELOAD", lib_path)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to start command");

    let status: ExitStatus = child.wait().expect("Failed to wait for child process");

    if status.success() {
        println!("Command executed successfully");
    } else {
        println!("Command failed with exit code: {:?}", status.code());
    }
}

#[cfg(target_os = "windows")]
fn launch(lib_path: &str, exe_cmdline: &str) {
    eprintln!("Windows is not supported yet.")
}

#[cfg(target_os = "macos")]
fn launch(lib_path: &str, exe_cmdline: &str) {
    eprintln!("macOS is not supported yet.")
}

// 非以上系统
#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
fn launch(lib_path: &str, exe_cmdline: &str) {
    eprintln!("Current platform is not supported.");
}

fn main() {
    println!("{}", LIB_FILENAME);
    println!("length: {}", LIB_FILE.len());

    let exe_cmdline = std::env::args().nth(1).expect("no exe_cmdline provided");

    let tmp_dir = tempfile::tempdir().unwrap();
    let tmp_dir_path = tmp_dir.path();
    let lib_output_path = tmp_dir_path.join(LIB_FILENAME);
    std::fs::write(&lib_output_path, LIB_FILE).unwrap();
    let lib_output_path_str = lib_output_path.to_str().unwrap();
    println!("lib_output_path: {}", lib_output_path_str);

    launch(lib_output_path_str, &exe_cmdline);

    tmp_dir.close().expect("failed to close temp dir");
}
