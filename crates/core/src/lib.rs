use std::ffi::c_void;

use ctor::ctor;
use frida_gum::{Gum, interceptor::Interceptor, Module};
use frida_gum::interceptor::{InvocationContext, InvocationListener};
use lazy_static::lazy_static;
use crate::platform::V8_SCRIPT_COMPILER_COMPILE_FUNCTION_INTERNAL_SYMBOL;

use crate::utils::{context_get_isolate, patch_source_if_needed, Source};

mod utils;
mod platform;

lazy_static! {
    static ref GUM: Gum = unsafe { Gum::obtain() };
}

// struct V8ScriptCompilerCompileUnboundInternalListener;

// impl InvocationListener for V8ScriptCompilerCompileUnboundInternalListener {
//     fn on_enter(&mut self, frida_context: InvocationContext) {
//         unsafe {
//             let isolate = frida_context.arg(0) as *const c_void;
//             let source = (frida_context.arg(1) as *mut Source).as_mut().unwrap();
//             patch_source_if_needed(isolate, source);
//         }
//     }
//
//     fn on_leave(&mut self, _frida_context: InvocationContext) {}
// }

// v8::ScriptCompiler::CompileFunctionInternal(v8::Local<v8::Context>, v8::ScriptCompiler::Source*, unsigned long, v8::Local<v8::String>*, unsigned long, v8::Local<v8::Object>*, v8::ScriptCompiler::CompileOptions, v8::ScriptCompiler::NoCacheReason, v8::Local<v8::ScriptOrModule>*)
struct V8ScriptCompilerCompileFunctionInternalListener;

impl InvocationListener for V8ScriptCompilerCompileFunctionInternalListener {
    fn on_enter(&mut self, frida_context: InvocationContext) {
        unsafe {
            #[cfg(target_os = "linux")]
            let context = frida_context.arg(0) as *const c_void;
            #[cfg(target_os = "linux")]
            let source = (frida_context.arg(1) as *mut Source).as_mut().unwrap();
            #[cfg(target_os = "windows")]
            let context = frida_context.arg(1) as *const c_void;
            #[cfg(target_os = "windows")]
            let source = (frida_context.arg(2) as *mut Source).as_mut().unwrap();

            let isolate = context_get_isolate(context);
            patch_source_if_needed(isolate, source);
        }
    }

    fn on_leave(&mut self, _frida_context: InvocationContext) {}
}

#[ctor]
fn init() {
    let mut interceptor = Interceptor::obtain(&GUM);

    interceptor.begin_transaction();

    // let v8_script_compiler_compile_unbound_internal = Module::find_export_by_name(None, "_ZN2v814ScriptCompiler22CompileUnboundInternalEPNS_7IsolateEPNS0_6SourceENS0_14CompileOptionsENS0_13NoCacheReasonE");
    // match v8_script_compiler_compile_unbound_internal {
    //     None => {
    //         println!("[-] v8_script_compiler_compile_unbound_internal not found")
    //     }
    //     Some(addr) => {
    //         println!("[*] v8_script_compiler_compile_unbound_internal found: {:?}", addr.0);
    //         let mut v8_script_compiler_compile_unbound_internal_listener = V8ScriptCompilerCompileUnboundInternalListener;
    //         interceptor.attach(addr, &mut v8_script_compiler_compile_unbound_internal_listener);
    //     }
    // }

    let v8_script_compiler_compile_function_internal = Module::find_export_by_name(None, V8_SCRIPT_COMPILER_COMPILE_FUNCTION_INTERNAL_SYMBOL);

    match v8_script_compiler_compile_function_internal {
        None => {
            println!("[-] v8_script_compiler_compile_function_internal not found")
        }
        Some(addr) => {
            println!("[*] v8_script_compiler_compile_function_internal found: {:?}", addr.0);
            let mut v8_script_compiler_compile_function_internal_listener = V8ScriptCompilerCompileFunctionInternalListener;
            interceptor.attach(addr, &mut v8_script_compiler_compile_function_internal_listener);
        }
    }

    // for module in Module::enumerate_modules() {
    //     for export in Module::enumerate_exports(&module.name) {
    //         if export.name.contains("Compile") {
    //             let demangled_name = try_demangle(&export.name);
    //             if demangled_name.contains("v8::ScriptCompiler::Compile") {
    //                 println!("{}, {}", &module.name, &export.name);
    //                 println!("[*] compile func found: {:?}", demangled_name);
    //             }
    //         }
    //     }
    // }


    // let v8_compiler_compile = Module::find_export_by_name(None, "_ZN2v814ScriptCompiler7CompileENS_5LocalINS_7ContextEEEPNS0_6SourceENS0_14CompileOptionsENS0_13NoCacheReasonE");
    // match v8_compiler_compile {
    //     None => {
    //         println!("[-] v8_compiler_compile not found")
    //     }
    //     Some(addr) => {
    //         println!("[*] v8_compiler_compile found: {:?}", addr.0);
    //         let mut v8_script_compiler_compile_listener = V8ScriptCompilerCompileListener;
    //         interceptor.attach(addr, &mut v8_script_compiler_compile_listener);
    //     }
    // }

    interceptor.end_transaction();
}