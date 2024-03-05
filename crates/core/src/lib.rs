use std::path::Path;

use ctor::ctor;
use frida_gum::interceptor::{InvocationContext, InvocationListener};
use frida_gum::{interceptor::Interceptor, Gum};
use once_cell::sync::Lazy;
use tracing::*;
use tracing_subscriber::fmt::time::uptime;

use crate::core::process_script;
use crate::v8_sys::{V8Context, V8Source};
use crate::{
    config::{Config, ReadFromFile},
    identifier::Identifier,
};

mod config;
mod core;
mod identifier;
mod matcher;
mod processor;
mod source;
mod v8_sys;

static GUM: Lazy<Gum> = Lazy::new(|| unsafe { Gum::obtain() });

static CONFIG: Lazy<Config> = Lazy::new(|| {
    let config_file_path = std::env::var("V8_KILLER_CONFIG_FILE_PATH");
    match config_file_path {
        Ok(config_file_path) => {
            info!("V8_KILLER_CONFIG_FILE_PATH: {config_file_path}");
            let path = Path::new(&config_file_path);
            let config = Config::load_from_toml(path);
            info!("Read Config success: {config:#?}");
            config
        }
        Err(_) => {
            warn!("V8_KILLER_CONFIG_FILE_PATH not found");
            warn!("Please set V8_KILLER_CONFIG_FILE_PATH to config file path");
            warn!("V8 Killer will only tracing source code without config file");
            Default::default()
        }
    }
});

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
            process_script(&CONFIG, context, source);
        }
    }

    fn on_leave(&mut self, _frida_context: InvocationContext) {}
}

#[ctor]
fn init() {
    tracing_subscriber::fmt()
        .with_timer(uptime())
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Fix no output in the Windows GUI subsystem programs
    // See also: [#11](https://github.com/ShellWen/v8_killer/issues/11)
    #[cfg(target_os = "windows")]
    unsafe {
        use windows::Win32::System::Console::{AttachConsole, ATTACH_PARENT_PROCESS};

        let _ = AttachConsole(ATTACH_PARENT_PROCESS);
    }

    info!("V8 Killer has been injected and started!");

    let mut interceptor = Interceptor::obtain(&GUM);

    interceptor.begin_transaction();

    let v8_script_compiler_compile_function_internal = CONFIG
        .identifiers
        .V8_SCRIPT_COMPILER_COMPILE_FUNCTION_INTERNAL
        .identify();

    match v8_script_compiler_compile_function_internal {
        None => {
            error!("v8_script_compiler_compile_function_internal not found");
            error!("source processing will not work properly");
        }
        Some(addr) => {
            info!(
                "v8_script_compiler_compile_function_internal found: {:?}",
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
