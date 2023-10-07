use aobscan::PatternBuilder;
use frida_gum::{Module, NativePointer};
use lazy_static::lazy_static;
use std::ffi::c_void;

use crate::memory_region::MemoryRegion;
use crate::CONFIG;

#[cfg(any(target_os = "linux", target_os = "macos"))]
const V8_SCRIPT_COMPILER_COMPILE_FUNCTION_INTERNAL_SYMBOL: &str = "_ZN2v814ScriptCompiler23CompileFunctionInternalENS_5LocalINS_7ContextEEEPNS0_6SourceEmPNS1_INS_6StringEEEmPNS1_INS_6ObjectEEENS0_14CompileOptionsENS0_13NoCacheReasonEPNS1_INS_14ScriptOrModuleEEE";
#[cfg(target_os = "windows")]
const V8_SCRIPT_COMPILER_COMPILE_FUNCTION_INTERNAL_SYMBOL: &str = "?CompileFunctionInternal@ScriptCompiler@v8@@CA?AV?$MaybeLocal@VFunction@v8@@@2@V?$Local@VContext@v8@@@2@PEAVSource@12@_KQEAV?$Local@VString@v8@@@2@2QEAV?$Local@VObject@v8@@@2@W4CompileOptions@12@W4NoCacheReason@12@PEAV?$Local@VScriptOrModule@v8@@@2@@Z";

#[cfg(any(target_os = "linux", target_os = "macos"))]
const V8_STRING_UTF8LENGTH_SYMBOL: &str = "_ZNK2v86String10Utf8LengthEPNS_7IsolateE";
#[cfg(target_os = "windows")]
const V8_STRING_UTF8LENGTH_SYMBOL: &str = "?Utf8Length@String@v8@@QEBAHPEAVIsolate@2@@Z";

#[cfg(any(target_os = "linux", target_os = "macos"))]
const V8_STRING_WRITE_UTF8_SYMBOL: &str = "_ZNK2v86String9WriteUtf8EPNS_7IsolateEPciPii";
#[cfg(target_os = "windows")]
const V8_STRING_WRITE_UTF8_SYMBOL: &str = "?WriteUtf8@String@v8@@QEBAHPEAVIsolate@2@PEADHPEAHH@Z";

#[cfg(any(target_os = "linux", target_os = "macos"))]
const V8_CONTEXT_GET_ISOLATE_SYMBOL: &str = "_ZN2v87Context10GetIsolateEv";
#[cfg(target_os = "windows")]
const V8_CONTEXT_GET_ISOLATE_SYMBOL: &str = "?GetIsolate@Context@v8@@QEAAPEAVIsolate@2@XZ";

#[cfg(any(target_os = "linux", target_os = "macos"))]
const V8_STRING_NEW_FROM_UTF8_PTR: &str =
    "_ZN2v86String11NewFromUtf8EPNS_7IsolateEPKcNS_13NewStringTypeEi";
#[cfg(target_os = "windows")]
const V8_STRING_NEW_FROM_UTF8_PTR: &str = "?NewFromUtf8@String@v8@@SA?AV?$MaybeLocal@VString@v8@@@2@PEAVIsolate@2@PEBDW4NewStringType@2@H@Z";

pub struct Exports {
    pub v8_script_compiler_compile_function_internal: NativePointer,
    pub v8_string_utf8_length: NativePointer,
    pub v8_string_write_utf8: NativePointer,
    pub v8_context_get_isolate: NativePointer,
    pub v8_string_new_from_utf8: NativePointer,
}

// Unsafe, but we only use it in static context
unsafe impl Sync for Exports {}

fn find_export_by_name(export_name: &str) -> Option<NativePointer> {
    Module::find_export_by_name(None, export_name)
}

fn find_export_by_signature(signature: &str) -> Option<NativePointer> {
    let pattern = PatternBuilder::from_ida_style(signature)
        .unwrap()
        .with_all_threads()
        .build();
    let mut results = vec![];
    MEMORY_REGIONS.iter().for_each(|region| {
        let slice = region.to_slice();
        pattern.scan(&slice, |result| {
            results.push(region.start + result);
            true
        });
    });
    if results.len() > 0 {
        if results.len() > 1 {
            println!(
                "[!] Found {} results for signature {}, use first",
                results.len(),
                signature
            );
        }
        Some(NativePointer(results[0] as *mut c_void))
    } else {
        None
    }
}

fn find_export(export_name: Option<&str>, signature: Option<&str>) -> Option<NativePointer> {
    let export_found_by_name = match export_name {
        Some(export_name) => {
            println!("[*] Finding export {}...", export_name);
            let export = find_export_by_name(export_name);
            match export {
                Some(export) => {
                    println!("[+] Found export {} at {:p}", export_name, export.0);
                    Some(export)
                }
                None => {
                    println!("[-] Failed to find export {}", export_name);
                    None
                }
            }
        }
        None => None,
    };
    if export_found_by_name.is_some() {
        return export_found_by_name;
    }
    let export_found_by_signature = match signature {
        Some(signature) => {
            println!("[*] Finding export {} by signature...", signature);
            let export = find_export_by_signature(signature);
            match export {
                Some(export) => {
                    println!(
                        "[+] Found export {} at {:p} by signature",
                        signature, export.0
                    );
                    Some(export)
                }
                None => {
                    println!("[-] Failed to find export {} by signature", signature);
                    None
                }
            }
        }
        None => None,
    };
    if export_found_by_signature.is_some() {
        return export_found_by_signature;
    }

    None
}

fn find_all_exports() -> Exports {
    let config = unsafe { CONFIG.as_ref().unwrap() };
    let use_export_name = config.common.use_export_name;
    let use_sigscan = config.common.use_sigscan;

    let v8_script_compiler_compile_function_internal = find_export(
        if use_export_name {
            Some(V8_SCRIPT_COMPILER_COMPILE_FUNCTION_INTERNAL_SYMBOL)
        } else {
            None
        },
        if use_sigscan { Some("55 48 ?? ?? 41 ?? 41 ?? 41 ?? 49 ?? ?? 41 ?? 53 48 ?? ?? 48 ?? ?? ?? ?? ?? ?? 48 ?? ?? ?? 48 ?? ?? ?? ?? ?? ?? 48 ?? ?? ?? ?? ?? ?? 4c ?? ?? ?? ?? ?? ?? 4c ?? ?? ?? ?? ?? ?? 48 ?? ?? ?? ?? ?? ?? 64") } else { None },
    )
    .expect("Failed to find v8_script_compiler_compile_function_internal");
    let v8_string_utf8_length = find_export(
        if use_export_name {
            Some(V8_STRING_UTF8LENGTH_SYMBOL)
        } else {
            None
        },
        if use_sigscan {
            Some("48 8B 07 8B 40 0B C3")
        } else {
            None
        },
    )
    .expect("Failed to find v8_string_utf8_length");
    let v8_string_write_utf8 = find_export(
        if use_export_name {
            Some(V8_STRING_WRITE_UTF8_SYMBOL)
        } else {
            None
        },
        if use_sigscan { Some("55 48 ?? ?? 41 ?? 45 ?? ?? 41 ?? 41 ?? 41 ?? 49 ?? ?? 53 48 ?? ?? 48 ?? ?? ?? ?? ?? ?? 48 ?? ?? ?? ?? ?? ?? 4C ?? ?? ?? ?? ?? ?? 64 ?? ?? ?? ?? ?? ?? ?? ?? 48") } else { None },
    )
    .expect("Failed to find v8_string_write_utf8");
    let v8_context_get_isolate = find_export(
        if use_export_name {
            Some(V8_CONTEXT_GET_ISOLATE_SYMBOL)
        } else {
            None
        },
        if use_sigscan {
            Some("48 8B 07 48 25 00 00 FC FF 48 8B 40 10 48 ?? ?? ?? ?? ?? C3")
        } else {
            None
        },
    )
    .expect("Failed to find v8_context_get_isolate");
    let v8_string_new_from_utf8 = find_export(
        if use_export_name {
            Some(V8_STRING_NEW_FROM_UTF8_PTR)
        } else {
            None
        },
        if use_sigscan { Some("55 48 ?? ?? 41 ?? 53 48 ?? ?? 48 ?? ?? ?? 64 ?? ?? ?? ?? ?? ?? ?? ?? 48 ?? ?? ?? 31 ?? 48 ?? ?? ?? ?? ?? ?? 85 ?? 74 ?? 81 ?? ?? ?? ?? ?? 7E ?? 31 ??") } else { None },
    )
    .expect("Failed to find v8_string_new_from_utf8");

    Exports {
        v8_script_compiler_compile_function_internal,
        v8_string_utf8_length,
        v8_string_write_utf8,
        v8_context_get_isolate,
        v8_string_new_from_utf8,
    }
}

lazy_static! {
    static ref MEMORY_REGIONS: Vec<MemoryRegion> = MemoryRegion::from_executable().unwrap();
    pub static ref EXPORTS: Exports = find_all_exports();
}
