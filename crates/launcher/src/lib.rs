use std::error::Error;

pub fn default_lib_filename<'a>() -> Result<&'a str, Box<dyn Error>> {
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

#[cfg(target_os = "linux")]
pub fn launch(lib_path: &str, executable: &str, args: &[String]) {
    use std::process::Command;
    use std::process::ExitStatus;
    use std::process::Stdio;

    let mut child = Command::new(executable)
        .args(args)
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
pub fn launch(lib_path: &str, executable: &str, args: &[String]) {
    use std::ffi::c_void;
    use windows::core::{s, w};
    use windows::core::PWSTR;
    use windows::Win32::Foundation::TRUE;
    use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;
    use windows::Win32::System::LibraryLoader::{GetModuleHandleW, GetProcAddress};
    use windows::Win32::System::Memory::{VirtualAllocEx, MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE};
    use windows::Win32::System::Threading::{
        CreateProcessW, CreateRemoteThread, ResumeThread, WaitForSingleObject, CREATE_SUSPENDED,
        INFINITE, LPTHREAD_START_ROUTINE, PROCESS_INFORMATION,
    };

    fn utf16_vec_from_str(str: String) -> Vec<u16> {
        let utf16 = str.encode_utf16().collect::<Vec<u16>>();
        let mut ret = Vec::<u16>::with_capacity(utf16.len() + 1);
        ret.extend(utf16);
        ret.push(0u16);
        ret
    }

    fn get_pwstr_length(pwstr: PWSTR) -> usize {
        let mut len = 0usize;
        while unsafe { *pwstr.0.offset(len.try_into().unwrap()) } != 0 {
            len += 1;
        }
        len
    }

    unsafe {
        let args_str: String = format!(
            "\"{}\" {}",
            executable,
            args.iter()
                .map(|arg| arg.to_string())
                // .map(|arg| format!("\"{}\"", arg))
                .collect::<Vec<String>>()
                .join(" ")
        );
        let mut args_utf16_vec = utf16_vec_from_str(args_str);
        let args_pwstr = PWSTR::from_raw(args_utf16_vec.as_mut_ptr());
        let mut path_utf16_vec = utf16_vec_from_str(lib_path.to_string());
        let path_pwstr = PWSTR::from_raw(path_utf16_vec.as_mut_ptr());
        let path_utf16_zeroend_size = get_pwstr_length(path_pwstr) * 2 + 2;

        let mut process_info = PROCESS_INFORMATION::default();
        println!("[*] Creating process.");
        CreateProcessW(
            None,
            args_pwstr,
            None,
            None,
            TRUE,
            CREATE_SUSPENDED,
            None,
            None,
            &Default::default(),
            &mut process_info,
        )
        .expect("CreateProcessW calling failed");
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
            path_pwstr.0 as *const c_void,
            path_utf16_zeroend_size,
            None,
        )
        .expect("WriteProcessMemory called failed");
        // let r_func_addr = unsafe{GetProcAddress(
        //     GetModuleHandleA("kernel32.dll\0".as_ptr() as _),
        //     "LoadLibraryW\0".as_ptr() as _,
        // )};
        println!("[*] Getting LoadLibraryW address.");
        let kernel_handle = GetModuleHandleW(w!("kernel32.dll")).unwrap();
        let load_library_address =
            (GetProcAddress(kernel_handle, s!("LoadLibraryW")).unwrap()) as *const c_void;
        assert!(!load_library_address.is_null());
        println!("[*] Creating remote thread.");
        let load_remote_thread_handle = CreateRemoteThread(
            process_info.hProcess,
            None,
            0,
            LPTHREAD_START_ROUTINE::from(std::mem::transmute::<
                _,
                unsafe extern "system" fn(*mut c_void) -> u32,
            >(load_library_address)),
            Some(remote_memory),
            0,
            None,
        )
        .unwrap();
        println!("[*] Core lib inject success. Waiting for thread end.");
        WaitForSingleObject(load_remote_thread_handle, INFINITE);
        println!("[*] Thread ended. Resume original thread.");
        println!("[*] --- Following is the original process output ---");
        ResumeThread(process_info.hThread);
        WaitForSingleObject(process_info.hProcess, INFINITE);
    }
}

#[cfg(target_os = "macos")]
pub fn launch(lib_path: &str, exe_cmdline: &str) {
    eprintln!("macOS is not supported yet.");
}

// 非以上系统
#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
pub fn launch(lib_path: &str, exe_cmdline: &str) {
    eprintln!("Unsupported platform.");
}
