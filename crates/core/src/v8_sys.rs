#![allow(non_camel_case_types)]

use std::ffi::{c_char, c_int, c_void};

use crate::identifier::Symbols;
use crate::SYMBOLS;

pub(crate) type V8Context = c_void;
pub(crate) type V8Isolate = c_void;
pub(crate) type V8String = c_void;
pub(crate) type V8Local<T> = *const T;

#[repr(C)]
#[derive(Debug)]
pub(crate) struct V8Source {
    /*
    Local<String> source_string;

    // Origin information
    Local<Value> resource_name;
    int resource_line_offset;
    int resource_column_offset;
    ScriptOriginOptions resource_options;
    Local<Value> source_map_url;
    Local<Data> host_defined_options;

    // Cached data from previous compilation (if a kConsume*Cache flag is
    // set), or hold newly generated cache data (kProduce*Cache flags) are
    // set when calling a compile method.
    std::unique_ptr<CachedData> cached_data;
    std::unique_ptr<ConsumeCodeCacheTask> consume_cache_task;
     */
    pub _source_string: V8Local<V8String>,
    pub _resource_name: V8Local<V8String>,
    pub _resource_line_offset: c_int,
    pub _resource_column_offset: c_int,
    pub _resource_options: c_int,
    pub _source_map_url: *const V8Local<V8String>,
    pub _host_defined_options: *const c_void,
    pub _cached_data: *const c_void,
    pub _consume_cache_task: *const c_void,
}

type v8__Context__GetIsolate = unsafe extern "C" fn(context: *const V8Context) -> *const V8Isolate;
type v8__String__Utf8Length =
    unsafe extern "C" fn(this: *const V8String, isolate: *const V8Isolate) -> c_int;
type v8__String__WriteUtf8 = unsafe extern "C" fn(
    this: *const V8String,
    isolate: *const V8Isolate,
    buffer: *mut c_char,
    length: c_int,
    nchars_ref: *mut c_int,
    options: c_int,
) -> c_int;
#[cfg(target_os = "linux")]
type v8__String__NewFromUtf8 = unsafe extern "C" fn(
    isolate: *const V8Isolate,
    data: *const c_char,
    new_type: i32,
    length: i32,
) -> V8Local<V8String>;
#[cfg(target_os = "windows")]
type v8__String__NewFromUtf8 = unsafe extern "C" fn(
    arg0: *mut V8Local<V8String>,
    isolate: *const V8Isolate,
    data: *const c_char,
    new_type: i32,
    length: i32,
) -> *mut V8Local<V8String>;
#[cfg(target_os = "macos")]
type v8__String__NewFromUtf8 = unsafe extern "C" fn(
    isolate: *const V8Isolate,
    data: *const c_char,
    new_type: i32,
    length: i32,
) -> V8Local<V8String>;

pub(crate) unsafe fn v8_context_get_isolate(context: *const V8Context) -> *const V8Isolate {
    if let Some(pointer) = SYMBOLS
        .V8_ISOLATE_GET_CURRENT
        .filter(|_| SYMBOLS.V8_CONTEXT_GET_ISOLATE.is_none())
    {
        let function: unsafe extern "C" fn() -> *const V8Isolate = std::mem::transmute(pointer.0);
        return function();
    }
    let v8_context_get_isolate_ptr = SYMBOLS.V8_CONTEXT_GET_ISOLATE.unwrap();
    let v8_context_get_isolate_func: v8__Context__GetIsolate =
        std::mem::transmute(v8_context_get_isolate_ptr.0);

    v8_context_get_isolate_func(context)
}

pub(super) unsafe fn v8_string_utf8_length(
    symbols: &Symbols,
    this: *const V8String,
    isolate: *const V8Isolate,
) -> usize {
    if let Some(pointer) = symbols
        .V8_STRING_UTF8LENGTH_SIZE_T
        .filter(|_| symbols.V8_STRING_UTF8LENGTH.is_none())
    {
        let function: unsafe extern "C" fn(*const V8String, *const V8Isolate) -> usize =
            std::mem::transmute(pointer.0);
        return function(this, isolate);
    }
    let v8_string_utf8_length_ptr = symbols.V8_STRING_UTF8LENGTH.unwrap();
    let v8_string_utf8_length_func: v8__String__Utf8Length =
        std::mem::transmute(v8_string_utf8_length_ptr.0);

    v8_string_utf8_length_func(this, isolate)
        .try_into()
        .expect("negative V8 string length")
}

pub(crate) unsafe fn v8_string_write_utf8(
    symbols: &Symbols,
    this: *const V8String,
    isolate: *const V8Isolate,
    buffer: *mut c_char,
    length: c_int,
    nchars_ref: *mut c_int,
    options: c_int,
) -> c_int {
    let v8_string_write_utf8_ptr = symbols.V8_STRING_WRITE_UTF8.unwrap();
    let v8_string_write_utf8_func: v8__String__WriteUtf8 =
        std::mem::transmute(v8_string_write_utf8_ptr.0);

    v8_string_write_utf8_func(this, isolate, buffer, length, nchars_ref, options)
}

pub(crate) unsafe fn v8_string_new_from_utf8(
    isolate: *const V8Isolate,
    data: *const c_char,
    new_type: i32,
    length: i32,
) -> V8Local<V8String> {
    let v8_string_new_from_utf8_ptr = SYMBOLS.V8_STRING_NEW_FROM_UTF8.unwrap();
    let v8_string_new_from_utf8_func: v8__String__NewFromUtf8 =
        std::mem::transmute(v8_string_new_from_utf8_ptr.0);

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        v8_string_new_from_utf8_func(isolate, data, new_type, length)
    }
    #[cfg(target_os = "windows")]
    {
        let mut result = std::ptr::null();
        v8_string_new_from_utf8_func(&mut result, isolate, data, new_type, length);
        result
    }
}

pub(crate) fn string_from_local_string(
    isolate: *const V8Isolate,
    local_string: *const V8String,
) -> String {
    string_from_local_string_with_symbols(&SYMBOLS, isolate, local_string)
}

pub(crate) fn string_from_local_string_with_symbols(
    symbols: &Symbols,
    isolate: *const V8Isolate,
    local_string: *const V8String,
) -> String {
    unsafe {
        let length = v8_string_utf8_length(symbols, local_string, isolate);
        let mut buffer = vec![0u8; length];
        let written = if let Some(pointer) = symbols
            .V8_STRING_WRITE_UTF8_SIZE_T
            .filter(|_| symbols.V8_STRING_WRITE_UTF8.is_none())
        {
            let function: unsafe extern "C" fn(
                *const V8String,
                *const V8Isolate,
                *mut c_char,
                usize,
                c_int,
                *mut usize,
            ) -> usize = std::mem::transmute(pointer.0);
            function(
                local_string,
                isolate,
                buffer.as_mut_ptr().cast(),
                length,
                2,
                std::ptr::null_mut(),
            )
        } else {
            v8_string_write_utf8(
                symbols,
                local_string,
                isolate,
                buffer.as_mut_ptr().cast(),
                length.try_into().expect("V8 string too long"),
                std::ptr::null_mut(),
                2 | 8,
            ) as usize
        };
        buffer.truncate(written);
        String::from_utf8(buffer).expect("V8 returned invalid UTF-8")
    }
}

pub(crate) fn local_string_from_string(
    isolate: *const V8Isolate,
    string: String,
) -> V8Local<V8String> {
    unsafe {
        v8_string_new_from_utf8(
            isolate,
            string.as_ptr().cast(),
            0,
            string.len().try_into().expect("V8 string too long"),
        )
    }
}
