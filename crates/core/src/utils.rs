#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::ffi::{c_char, c_int, c_void};

use frida_gum::Module;

use crate::platform::{V8_CONTEXT_GET_ISOLATE_SYMBOL, V8_STRING_NEW_FROM_UTF8_PTR, V8_STRING_UTF8LENGTH_SYMBOL, V8_STRING_WRITE_UTF8_SYMBOL};

pub type v8__Context__GetIsolate = unsafe extern "C" fn(context: *const c_void) -> *const c_void;
pub type v8__String__Utf8Length = unsafe extern "C" fn(this: *const c_void, isolate: *const c_void) -> usize;
pub type v8__String__WriteUtf8 = unsafe extern "C" fn(this: *const c_void, isolate: *const c_void, buffer: *mut c_char, length: c_int, nchars_ref: *mut usize, options: c_int) -> c_int;
#[cfg(target_os = "linux")]
pub type v8__String__NewFromUtf8 = unsafe extern "C" fn(isolate: *const c_void, data: *const c_char, new_type: i32, length: i32) -> *const c_void;
// pub type v8__String__NewFromUtf8 = unsafe extern "C" fn(isolate: *const c_void, data: *const c_char) -> *const c_void;
#[cfg(target_os = "windows")]
pub type v8__String__NewFromUtf8 = unsafe extern "C" fn(arg0: *const *mut c_void, isolate: *const c_void, data: *const c_char, new_type: i32, length: i32) -> *const c_void;

#[repr(C)]
#[derive(Debug)]
pub struct Source {
    pub _source_string: *const c_void,
    pub _resource_name: *const c_void,
    pub _resource_line_offset: i32,
    pub _resource_column_offset: c_int,
    pub _resource_options: c_int,
    pub _source_map_url: *const c_void,
    pub _host_defined_options: *const c_void,
    pub _cached_data: *const c_void,
    pub _consume_cache_task: *const c_void,
    pub _compile_hint_callback: *const c_void,
    pub _compile_hint_callback_data: *const c_void,
}

pub fn cstr_from_string(s: String) -> *const c_char {
    std::ffi::CString::new(s).unwrap().into_raw()
}

pub unsafe fn char_vec_to_string(v: &Vec<c_char>) -> String {
    std::ffi::CStr::from_ptr(v.as_ptr()).to_str().unwrap().to_string()
}

pub unsafe fn string_from_local_string(isolate: *const c_void, local_string: *const c_void) -> String {
    let v8__String__Utf8Length_ptr = Module::find_export_by_name(None, V8_STRING_UTF8LENGTH_SYMBOL).unwrap();
    let v8__String__Utf8Length_func: v8__String__Utf8Length = std::mem::transmute(v8__String__Utf8Length_ptr.0);
    let v8__String__WriteUtf8_ptr = Module::find_export_by_name(None, V8_STRING_WRITE_UTF8_SYMBOL).unwrap();
    let v8__String__WriteUtf8_func: v8__String__WriteUtf8 = std::mem::transmute(v8__String__WriteUtf8_ptr.0);

    let length = v8__String__Utf8Length_func(local_string, isolate);
    // 我也不知道为什么要 +1，但是不 +1 的话就有可能 SIGSEGV ¯\_(ツ)_/¯
    // 反正不可能是因为 \0 的问题
    let mut buffer: Vec<c_char> = vec![0; length + 1];
    v8__String__WriteUtf8_func(local_string, isolate, buffer.as_mut_ptr(), -1, std::ptr::null_mut(), 0);
    char_vec_to_string(&buffer)
}

pub unsafe fn context_get_isolate(context: *const c_void) -> *const c_void {
    let v8__Context__GetIsolate_ptr = Module::find_export_by_name(None, V8_CONTEXT_GET_ISOLATE_SYMBOL).unwrap();
    let v8__Context__GetIsolate_func: v8__Context__GetIsolate = std::mem::transmute(v8__Context__GetIsolate_ptr.0);

    v8__Context__GetIsolate_func(context)
}

pub unsafe fn local_string_from_string(isolate: *const c_void, string: String) -> *const c_void {
    let v8__String__NewFromUtf8_ptr = Module::find_export_by_name(None, V8_STRING_NEW_FROM_UTF8_PTR).unwrap();
    let v8__String__NewFromUtf8_func: v8__String__NewFromUtf8 = std::mem::transmute(v8__String__NewFromUtf8_ptr.0);

    let s_ptr = cstr_from_string(string);
    #[cfg(target_os = "linux")]
    {
        v8__String__NewFromUtf8_func(isolate, s_ptr, 0, -1)
    }
    #[cfg(target_os = "windows")]
    {
        use std::ptr::null_mut;

        let mut arg0_value: *mut c_void = null_mut();
        let arg0: *const *mut c_void = &mut arg0_value;
        v8__String__NewFromUtf8_func(arg0, isolate, s_ptr, 0, -1);
        arg0_value
    }
}

pub fn is_resource_should_patch(resource_name: &str, inject_script_path: &str) -> bool {
    let default_keywords = vec![
        "app_launcher/index.js",
        "app/index.js",
    ];
    let keywords_env = std::env::var(
        "INJECT_KEYWORDS").unwrap_or("".to_string());
    let keywords_env_vec: Vec<&str> = keywords_env.split(";").collect();
    // 对每一个 keyword 执行 .trim()，然后过滤掉空字符串
    let keywords_env_vec: Vec<&str> = keywords_env_vec.iter().map(|&keyword| keyword.trim()).filter(|&keyword| keyword != "").collect();
    // 合并两个列表
    let keywords = [default_keywords, keywords_env_vec].concat();
    return if resource_name == inject_script_path {
        false
    } else {
        keywords.iter().any(|&keyword| {
            resource_name.contains(keyword)
        })
    };
}

pub fn patch_script(original: &str, inject_script_path: &str) -> String {
    let inject_script_content = std::fs::read_to_string(inject_script_path).expect("Something went wrong reading the file");
    let prefix = include_str!("../assets/prefix.js").replace("// <!INJECT_SCRIPT!>", &inject_script_content);
    prefix + original
}

pub unsafe fn patch_source_if_needed(isolate: *const c_void, source: &mut Source) -> bool {
    let resource_name: String = match source._resource_name {
        r if r.is_null() => "<unknown>".to_string(),
        r => string_from_local_string(isolate, r),
    };

    let inject_script_path = std::env::var("INJECT_SCRIPT_PATH").expect("INJECT_SCRIPT_PATH is not defined");

    match resource_name {
        r if is_resource_should_patch(&r, &inject_script_path) => {
            println!("[*] patching source for {}", r);
            let source_string = string_from_local_string(isolate, source._source_string);
            let patched = patch_script(&source_string, &inject_script_path);
            let patched_local_string = local_string_from_string(isolate, patched.to_string());
            source._source_string = patched_local_string;
            true
        }
        _ => false
    }
}
