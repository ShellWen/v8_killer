use clap::*;
use tracing::*;
use tracing_subscriber::fmt::time::uptime;

use std::env::current_exe;
use std::process::exit;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;
use v8_killer_launcher::{default_lib_filename, launch};

/// A simple launcher/injector for V8 Killer
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    /// Custom dynamic library to inject
    #[arg(long)]
    lib_name: Option<String>,
    /// Custom configuration file, will pass to the executable by environment variable `V8_KILLER_CONFIG_FILE_PATH`
    #[arg(long)]
    config: Option<String>,
    /// The executable to launch and inject dynamic library
    executable: String,
    /// The arguments for the executable
    #[arg(last = true)]
    arguments: Vec<String>,
}

fn main() {
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();
    tracing_subscriber::fmt()
        .with_timer(uptime())
        .with_env_filter(filter)
        .init();
    let args = Arguments::parse();
    if let Some(config) = &args.config {
        std::env::set_var("V8_KILLER_CONFIG_FILE_PATH", config);
    }
    let lib_filename = if let Some(lib_name) = args.lib_name {
        lib_name
    } else {
        default_lib_filename().to_owned()
    };
    let lib_path = current_exe().unwrap().parent().unwrap().join(lib_filename);
    let lib_path_str = lib_path.to_str().unwrap();

    info!("Executable: {}", args.executable);
    info!("Args: {:?}", args.arguments);
    info!("Core lib path: {}", lib_path_str);
    let command_args: Vec<&str> = args.arguments.iter().map(String::as_str).collect();
    let exit_status = launch(&args.executable, command_args.as_slice(), lib_path_str);
    if exit_status.success() {
        info!("Process exited successfully");
    } else {
        error!("Process exited with code: {:?}", exit_status.code());
        exit(exit_status.code().unwrap_or(1));
    }
}
