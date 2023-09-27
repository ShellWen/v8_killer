use std::error::Error;
use std::env::current_exe;

#[cfg(target_os = "linux")]
fn launch(lib_path: &str, exe_cmdline: &str) {
    use std::process::Command;
    use std::process::Stdio;
    use std::process::ExitStatus;

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
    unsafe {
        use std::ffi::c_void;
        use windows::core::{
            s, w,
        };
        use windows::Win32::Foundation::TRUE;
        use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;
        use windows::Win32::System::Memory::{
            MEM_COMMIT,
            MEM_RESERVE,
            PAGE_READWRITE,
            VirtualAllocEx,
        };
        use windows::Win32::System::Threading::{
            CREATE_SUSPENDED,
            CreateProcessW,
            CreateRemoteThread,
            INFINITE,
            LPTHREAD_START_ROUTINE,
            PROCESS_INFORMATION,
            ResumeThread,
            WaitForSingleObject,
        };
        use windows::Win32::System::LibraryLoader::{
            GetModuleHandleW,
            GetProcAddress,
        };
        use windows::core::PWSTR;

        let mut path_utf16_zeroend = lib_path.encode_utf16().collect::<Vec<u16>>();
        // \0 终止符
        path_utf16_zeroend.push(0);
        // UTF-16 的字节数
        let path_utf16_zeroend_size = path_utf16_zeroend.len() * 2;

        let mut exe_cmdline_utf16_vec = exe_cmdline.encode_utf16().collect::<Vec<u16>>();
        // \0 终止符
        exe_cmdline_utf16_vec.push(0);
        let exe_cmdline_pwstr = PWSTR::from_raw(exe_cmdline_utf16_vec.as_mut_ptr());
        let mut process_info = PROCESS_INFORMATION::default();
        println!("[*] Creating process.");
        CreateProcessW(
            None,
            exe_cmdline_pwstr,
            None,
            None,
            TRUE,
            CREATE_SUSPENDED,
            None,
            None,
            &Default::default(),
            &mut process_info,
        ).expect("CreateProcessW calling failed");
        println!("[*] PID: {}", process_info.dwProcessId);
        println!("[*] Alloc core lib path memory.");
        let remote_memory = VirtualAllocEx(
            process_info.hProcess,
            None,
            path_utf16_zeroend_size,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        );
        assert!(!remote_memory.is_null());
        println!("[*] Remote lib path memory Address: {:p}.", remote_memory);
        println!("[*] Writing core lib path to process.");
        WriteProcessMemory(
            process_info.hProcess,
            remote_memory,
            path_utf16_zeroend.as_ptr() as *const c_void,
            path_utf16_zeroend_size,
            None,
        ).expect("WriteProcessMemory called failed");
        // let r_func_addr = unsafe{GetProcAddress(
        //     GetModuleHandleA("kernel32.dll\0".as_ptr() as _),
        //     "LoadLibraryW\0".as_ptr() as _,
        // )};
        println!("[*] Getting LoadLibraryW address.");
        let kernel_handle = GetModuleHandleW(
            w!("kernel32.dll")
        ).unwrap();
        let load_library_address = (GetProcAddress(
            kernel_handle,
            s!("LoadLibraryW"),
        ).unwrap()) as *const c_void;
        assert!(!load_library_address.is_null());
        println!("[*] Creating remote thread.");
        let load_remote_thread_handle = CreateRemoteThread(
            process_info.hProcess,
            None,
            0,
            LPTHREAD_START_ROUTINE::from(
                std::mem::transmute::<_, unsafe extern "system" fn(*mut c_void) -> u32>(
                    load_library_address)),
            Some(remote_memory),
            0,
            None,
        ).unwrap();
        println!("[*] Core lib inject success. Waiting for thread end.");
        WaitForSingleObject(
            load_remote_thread_handle,
            INFINITE,
        );
        println!("[*] Thread ended. Resume original thread.");
        println!("[*] --- Following is the original process output ---");
        ResumeThread(process_info.hThread);
        WaitForSingleObject(
            process_info.hProcess,
            INFINITE,
        );
    }
}

#[cfg(target_os = "macos")]
fn launch(lib_path: &str, exe_cmdline: &str) {
    eprintln!("macOS is not supported yet.");
}

// 非以上系统
#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
fn launch(lib_path: &str, exe_cmdline: &str) {
    eprintln!("Unsupported platform.");
}

fn default_lib_filename<'a>() -> Result<&'a str, Box<dyn Error>> {
    #[cfg(target_os = "linux")]
    {
        Ok("libv8_killer_core.so")
    }

    #[cfg(target_os = "windows")]
    {
        Ok("v8_killer_core.dll")
    }

    #[cfg(target_os = "macos")]
    {
        // TODO: not sure
        Ok("libv8_killer_core.dylib")
    }

    // 默认情况，如果没有匹配的操作系统，则返回一个合适的默认值
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        Err("Unsupported platform".into())
    }
}

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
