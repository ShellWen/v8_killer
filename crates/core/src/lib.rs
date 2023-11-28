use std::path::Path;

use ctor::ctor;
use frida_gum::interceptor::{InvocationContext, InvocationListener};
use frida_gum::{interceptor::Interceptor, Gum};
use lazy_static::lazy_static;

use crate::config::{Config, ReadFromFile};
use crate::core::process_script;
use crate::identifier::Identifier;
use crate::v8_sys::{V8Context, V8Source};

mod config;
mod core;
mod identifier;
mod matcher;
mod processor;
mod source;
mod v8_sys;

lazy_static! {
    static ref GUM: Gum = unsafe { Gum::obtain() };
}

static mut CONFIG: Option<Config> = None;

// v8::ScriptCompiler::CompileFunctionInternal(v8::Local<v8::Context>, v8::ScriptCompiler::Source*, unsigned long, v8::Local<v8::String>*, unsigned long, v8::Local<v8::Object>*, v8::ScriptCompiler::CompileOptions, v8::ScriptCompiler::NoCacheReason, v8::Local<v8::ScriptOrModule>*)
struct V8ScriptCompilerCompileFunctionInternalListener;

impl InvocationListener for V8ScriptCompilerCompileFunctionInternalListener {
    fn on_enter(&mut self, frida_context: InvocationContext) {
        unsafe {
            #[cfg(any(target_os = "linux", target_os = "macos"))]
            let context = frida_context.arg(0) as *const V8Context;
            #[cfg(any(target_os = "linux", target_os = "macos"))]
            let source = frida_context.arg(1) as *mut V8Source;
            #[cfg(target_os = "windows")]
            let context = frida_context.arg(1) as *const V8Context;
            #[cfg(target_os = "windows")]
            let source = frida_context.arg(2) as *mut V8Source;
            let config = CONFIG.as_ref().unwrap();
            process_script(config, context, source);
        }
    }

    fn on_leave(&mut self, _frida_context: InvocationContext) {}
}

#[ctor]
fn init() {
    // Fix no output in the Windows GUI subsystem programs
    // See also: [#11](https://github.com/ShellWen/v8_killer/issues/11)
    #[cfg(target_os = "windows")]
    unsafe {
        use windows::Win32::System::Console::{AttachConsole, ATTACH_PARENT_PROCESS};

        let _ = AttachConsole(ATTACH_PARENT_PROCESS);
    }

    // 读取环境变量
    let config_file_path = std::env::var("V8_KILLER_CONFIG_FILE_PATH");
    match config_file_path {
        Ok(config_file_path) => {
            println!("[*] V8_KILLER_CONFIG_FILE_PATH: {}", config_file_path);
            let path = Path::new(&config_file_path);
            let config = Config::load_from_toml(path);
            println!("[*] Read Config success");
            println!("[*] Config: {:?}", config);
            unsafe {
                CONFIG = Some(config);
            }
            let mut interceptor = Interceptor::obtain(&GUM);

            interceptor.begin_transaction();

            let v8_script_compiler_compile_function_internal = unsafe { CONFIG.as_ref().unwrap() }
                .identifiers
                .V8_SCRIPT_COMPILER_COMPILE_FUNCTION_INTERNAL
                .identify();

            match v8_script_compiler_compile_function_internal {
                None => {
                    println!("[-] v8_script_compiler_compile_function_internal not found")
                }
                Some(addr) => {
                    println!(
                        "[*] v8_script_compiler_compile_function_internal found: {:?}",
                        addr.0
                    );
                    let mut v8_script_compiler_compile_function_internal_listener =
                        V8ScriptCompilerCompileFunctionInternalListener;
                    interceptor.attach(
                        addr,
                        &mut v8_script_compiler_compile_function_internal_listener,
                    );
                }
            }

            interceptor.end_transaction();
        }
        Err(_) => {
            println!("[-] WARN: V8_KILLER_CONFIG_FILE_PATH not found");
            println!("[-] WARN: Please set V8_KILLER_CONFIG_FILE_PATH to config file path");
            println!("[-] WARN: Without config file, V8 Killer will do nothing");
        }
    }
}
