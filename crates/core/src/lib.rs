use ctor::ctor;
use frida_gum::interceptor::{InvocationContext, InvocationListener};
use frida_gum::{interceptor::Interceptor, Gum};
use once_cell::sync::Lazy;
use std::path::Path;
use std::process;
use tracing::*;
use tracing_subscriber::fmt::time::uptime;

use crate::config::{Config, ReadFromFile};
use crate::core::process_script;
use crate::identifier::Symbols;
use crate::v8_sys::{V8Context, V8Source};

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

static SYMBOLS: Lazy<Symbols> = Lazy::new(|| {
    let symbols = Symbols::from_identifiers(&CONFIG.identifiers);
    info!("Symbols: {symbols:#?}");
    symbols
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

#[cfg(target_os = "linux")]
fn read_host_pid() -> Option<u32> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    // format:
    // NSpid:  1381510 1
    let mut nspid_line = status
        .lines()
        .find(|line| line.starts_with("NSpid"))?
        .split_whitespace();
    let nspid = nspid_line.nth(1)?.parse::<u32>().ok()?;
    // if ns pid is None, it will return None
    nspid_line.next()?;
    Some(nspid)
}

#[ctor]
fn init() {
    tracing_subscriber::fmt()
        .with_timer(uptime())
        .with_max_level(Level::DEBUG)
        .event_format(tracing_subscriber::fmt::format::Format::default())
        .init();

    let pid = process::id();
    #[cfg(not(target_os = "linux"))]
    let pid_span: Span = info_span!("process", pid);
    #[cfg(target_os = "linux")]
    let pid_span: Span = {
        let host_pid = read_host_pid();
        match host_pid {
            Some(host_pid) => info_span!("process", pid, host_pid, in_sandbox = true),
            None => info_span!("process", pid),
        }
    };
    let _enter = pid_span.enter();

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

    let v8_script_compiler_compile_function_internal =
        SYMBOLS.V8_SCRIPT_COMPILER_COMPILE_FUNCTION_INTERNAL;

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
