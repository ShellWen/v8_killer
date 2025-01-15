use std::process::ExitStatus;

pub fn default_lib_filename() -> &'static str {
    #[cfg(target_os = "linux")]
    return "libv8_killer_core.so";

    #[cfg(target_os = "windows")]
    return "v8_killer_core.dll";

    #[cfg(target_os = "macos")]
    return "libv8_killer_core.dylib";

    // unsupported target_os leads to a compile-time error
}

#[cfg(target_os = "windows")]
fn launch_with_remote_thread_inject(executable: &str, args: &[&str], lib_path: &str) -> ExitStatus {
    use std::ffi::c_void;
    use std::os::windows::process::ExitStatusExt;
    use std::thread::sleep;
    use std::time::Duration;
    use tracing::*;
    use windows::core::PWSTR;
    use windows::core::{s, w};
    use windows::Win32::Foundation::FARPROC;
    use windows::Win32::Foundation::STILL_ACTIVE;
    use windows::Win32::Foundation::TRUE;
    use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;
    use windows::Win32::System::LibraryLoader::{GetModuleHandleW, GetProcAddress};
    use windows::Win32::System::Memory::{VirtualAllocEx, MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE};
    use windows::Win32::System::Threading::GetExitCodeProcess;
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
    debug!("Creating process.");
    unsafe {
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
    }
    info!("PID: {}", process_info.dwProcessId);
    debug!("Alloc core lib path memory.");
    let remote_lib_path_memory = unsafe {
        VirtualAllocEx(
            process_info.hProcess,
            None,
            path_utf16_zeroend_size,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        )
    };
    assert!(!remote_lib_path_memory.is_null());
    debug!(
        "Remote lib path memory Address: {:p}.",
        remote_lib_path_memory
    );
    debug!("Writing core lib path to process.");
    unsafe {
        WriteProcessMemory(
            process_info.hProcess,
            remote_lib_path_memory,
            path_pwstr.0 as *const c_void,
            path_utf16_zeroend_size,
            None,
        )
        .expect("WriteProcessMemory called failed");
    }

    debug!("Getting LoadLibraryW address.");
    let kernel_handle = unsafe { GetModuleHandleW(w!("kernel32.dll")) }.unwrap();
    // it means FARPROC but with a value, equivalent to FARPROC.unwrap() when FARPROC has a value
    #[allow(non_camel_case_types)]
    type FARPROC_Value = unsafe extern "system" fn() -> isize;
    let load_library_address: FARPROC_Value =
        unsafe { GetProcAddress(kernel_handle, s!("LoadLibraryW")) }
            .expect("GetProcAddress of LoadLibraryW failed");
    debug!("Creating remote thread.");
    let load_remote_thread_handle = unsafe {
        CreateRemoteThread(
            process_info.hProcess,
            None,
            0,
            LPTHREAD_START_ROUTINE::from(std::mem::transmute::<
                FARPROC_Value,
                unsafe extern "system" fn(*mut c_void) -> u32,
            >(load_library_address)),
            Some(remote_lib_path_memory), // LoadLibraryW(lpLibFileName)
            0,
            None,
        )
    }
    .unwrap();
    info!("Core lib inject success. Waiting for thread end.");
    unsafe {
        WaitForSingleObject(load_remote_thread_handle, INFINITE);
    }
    info!("Thread ended. Resume original thread.");
    info!("--- Following is the original process output ---");
    unsafe {
        ResumeThread(process_info.hThread);
    }
    unsafe {
        WaitForSingleObject(process_info.hProcess, INFINITE);
    }
    let mut exit_code: u32 = 0;
    loop {
        unsafe {
            GetExitCodeProcess(process_info.hProcess, &mut exit_code)
                .expect("GetExitCodeProcess failed");
        }
        if exit_code != STILL_ACTIVE.0.into() {
            break;
        }
        warn!("Process is still running even after WaitForSingleObject. This should not happen. Waiting for 500ms.");
        sleep(Duration::from_millis(500)); // Why 500ms? Because I'm lazy
    }
    ExitStatus::from_raw(exit_code)
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn launch_with_env(executable: &str, args: &[&str], env: &[(&str, &str)]) -> ExitStatus {
    use std::process::Command;
    use std::process::Stdio;

    let mut command = Command::new(executable);
    command
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    for (key, value) in env {
        command.env(key, value);
    }
    let mut child = command.spawn().expect("Failed to start command");

    child.wait().expect("Failed to wait for child process")
}

#[allow(unreachable_code)]
pub fn launch(executable: &str, args: &[&str], lib_path: &str) -> ExitStatus {
    #[cfg(target_os = "windows")]
    {
        return launch_with_remote_thread_inject(executable, args, lib_path);
    }
    #[cfg(target_os = "linux")]
    {
        return launch_with_env(executable, args, &[("LD_PRELOAD", lib_path)]);
    }
    #[cfg(target_os = "macos")]
    {
        return launch_with_env(executable, args, &[("DYLD_INSERT_LIBRARIES", lib_path)]);
    }

    unreachable!("Unsupported platform.");
}
