use crate::config::{Config, ReadFromFile};
use crate::core::process_script;
use crate::identifier::Symbols;
use crate::pid_span::pid_span;
use crate::v8_sys::{V8Context, V8Source};
use ctor::ctor;
use std::path::Path;
use std::sync::LazyLock;
use tracing::level_filters::LevelFilter;
use tracing::*;
use tracing_subscriber::fmt::time::uptime;
use tracing_subscriber::EnvFilter;

mod config;
mod core;
mod identifier;
mod matcher;
mod pid_span;
mod processor;
mod source;
mod v8_sys;

unsafe extern "C" {
    fn v8_killer_instrument(address: *mut std::ffi::c_void) -> std::ffi::c_int;
}

static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let config_file_path = std::env::var("V8_KILLER_CONFIG_FILE_PATH");
    match config_file_path {
        Ok(config_file_path) => {
            debug!("V8_KILLER_CONFIG_FILE_PATH: {config_file_path}");
            let path = Path::new(&config_file_path);
            let config = Config::load_from_toml(path);
            info!("Read config success");
            debug!("Config: {config:#?}");
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

static SYMBOLS: LazyLock<Symbols> = LazyLock::new(|| {
    let symbols = Symbols::from_identifiers(&CONFIG.identifiers);
    debug!("Symbols: {symbols:#?}");
    symbols
});

#[unsafe(no_mangle)]
unsafe extern "C" fn v8_killer_process(context: *const V8Context, source: *mut V8Source) {
    let span = pid_span();
    let _enter = span.enter();
    process_script(&CONFIG, context, source);
}

#[ctor(unsafe)]
fn init() {
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::fmt()
        .with_timer(uptime())
        .with_max_level(Level::DEBUG)
        .with_env_filter(filter)
        .init();

    let pid_span = pid_span();
    let _enter = pid_span.enter();

    // Fix no output in the Windows GUI subsystem programs
    // See also: [#11](https://github.com/ShellWen/v8_killer/issues/11)
    #[cfg(target_os = "windows")]
    unsafe {
        use windows_sys::Win32::System::Console::{AttachConsole, ATTACH_PARENT_PROCESS};

        AttachConsole(ATTACH_PARENT_PROCESS);
    }

    info!("V8 Killer has been injected and started!");

    if !SYMBOLS.is_complete() {
        error!(
            "Required V8 symbols not found; source processing is disabled: {:#?}",
            *SYMBOLS
        );
        return;
    }

    let address = SYMBOLS.V8_SCRIPT_COMPILER_COMPILE_FUNCTION.unwrap().0;
    let status = unsafe { v8_killer_instrument(address) };
    if status != 0 {
        error!("DobbyInstrument failed ({status}); source processing is disabled");
    }
}
