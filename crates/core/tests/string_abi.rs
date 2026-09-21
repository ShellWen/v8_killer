#![allow(dead_code)]

#[path = "../src/identifier.rs"]
mod identifier;
#[path = "../src/v8_sys.rs"]
mod v8_sys;

use identifier::{Identifiers, Symbols};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{c_char, c_int, c_void, CStr};
use std::sync::LazyLock;

static SYMBOLS: LazyLock<Symbols> = LazyLock::new(|| panic!("unexpected global symbol lookup"));

thread_local! {
    static EXPORTS: RefCell<HashMap<String, *mut c_void>> = RefCell::new(HashMap::new());
}

#[unsafe(no_mangle)]
unsafe extern "C" fn v8_killer_symbol(name: *const c_char) -> *mut c_void {
    let name = CStr::from_ptr(name).to_str().unwrap();
    EXPORTS.with(|exports| exports.borrow().get(name).copied().unwrap_or_default())
}

#[unsafe(no_mangle)]
unsafe extern "C" fn v8_killer_module(name: *const c_char) -> *mut c_void {
    let name = CStr::from_ptr(name).to_str().unwrap();
    let executable = std::env::current_exe().unwrap();
    if Some(name) == executable.to_str()
        || Some(name) == executable.file_name().and_then(|name| name.to_str())
    {
        return 0x1000usize as *mut c_void;
    }
    std::ptr::null_mut()
}

unsafe extern "C" fn length_legacy(this: *const c_void, _: *const c_void) -> c_int {
    (&*(this as *const String)).len().try_into().unwrap()
}

unsafe extern "C" fn length_modern(this: *const c_void, _: *const c_void) -> usize {
    (&*(this as *const String)).len()
}

unsafe extern "C" fn write_legacy(
    this: *const c_void,
    _: *const c_void,
    buffer: *mut c_char,
    length: c_int,
    nchars: *mut c_int,
    options: c_int,
) -> c_int {
    let text = &*(this as *const String);
    assert_eq!(length as usize, text.len());
    assert!(nchars.is_null());
    assert_eq!(options, 2 | 8);
    std::ptr::copy_nonoverlapping(text.as_ptr(), buffer.cast(), text.len());
    length
}

unsafe extern "C" fn write_modern(
    this: *const c_void,
    _: *const c_void,
    buffer: *mut c_char,
    capacity: usize,
    flags: c_int,
    processed_characters: *mut usize,
) -> usize {
    let text = &*(this as *const String);
    assert_eq!(capacity, text.len());
    assert_eq!(flags, 2);
    assert!(processed_characters.is_null());
    std::ptr::copy_nonoverlapping(text.as_ptr(), buffer.cast(), text.len());
    text.len()
}

fn export(name: &str, pointer: *mut c_void) {
    EXPORTS.with(|exports| exports.borrow_mut().insert(name.into(), pointer));
}

#[test]
fn utf8_legacy_v2_and_renamed_selection() {
    let legacy_length = "_ZNK2v86String10Utf8LengthEPNS_7IsolateE";
    let legacy_writer = "_ZNK2v86String9WriteUtf8EPNS_7IsolateEPciPii";
    let v2_length = "_ZNK2v86String12Utf8LengthV2EPNS_7IsolateE";
    let v2_writer = "_ZNK2v86String11WriteUtf8V2EPNS_7IsolateEPcmiPm";
    let renamed_writer = "_ZNK2v86String9WriteUtf8EPNS_7IsolateEPcmiPm";
    for (length, writer, modern) in [
        (legacy_length, legacy_writer, false),
        (v2_length, v2_writer, true),
        (legacy_length, renamed_writer, true),
    ] {
        EXPORTS.with(|exports| exports.borrow_mut().clear());
        export(
            length,
            if modern {
                length_modern as *mut c_void
            } else {
                length_legacy as *mut c_void
            },
        );
        export(
            writer,
            if modern {
                write_modern as *mut c_void
            } else {
                write_legacy as *mut c_void
            },
        );
        let symbols = Symbols::from_identifiers(&Identifiers::default());
        assert_eq!(symbols.V8_STRING_WRITE_UTF8.is_none(), modern);
        assert_eq!(symbols.V8_STRING_UTF8LENGTH.is_none(), modern);
        for text in ["", "ASCII", "中文😀\0end"] {
            let text = text.to_owned();
            assert_eq!(
                v8_sys::string_from_local_string_with_symbols(
                    &symbols,
                    std::ptr::null(),
                    (&text as *const String).cast()
                ),
                text
            );
        }
    }

    export(v2_writer, write_legacy as *mut c_void);
    export(v2_length, length_legacy as *mut c_void);
    let symbols = Symbols::from_identifiers(&Identifiers::default());
    assert_eq!(
        symbols.V8_STRING_WRITE_UTF8_SIZE_T.unwrap().0,
        write_legacy as *mut c_void
    );

    export("custom_length", length_legacy as *mut c_void);
    export("custom_writer", write_legacy as *mut c_void);
    let identifiers = Identifiers {
        V8_STRING_UTF8LENGTH: vec![identifier::IdentifierEnum::SymbolIdentifier(
            identifier::SymbolIdentifier {
                symbols: vec!["custom_length".into()],
            },
        )],
        V8_STRING_WRITE_UTF8: vec![identifier::IdentifierEnum::SymbolIdentifier(
            identifier::SymbolIdentifier {
                symbols: vec!["custom_writer".into()],
            },
        )],
        ..Identifiers::default()
    };
    let symbols = Symbols::from_identifiers(&identifiers);
    let text = "自定义\0😀".to_owned();
    assert_eq!(
        v8_sys::string_from_local_string_with_symbols(
            &symbols,
            std::ptr::null(),
            (&text as *const String).cast()
        ),
        text
    );
}

#[test]
fn utf8_size_t_length_is_not_truncated() {
    unsafe extern "C" fn large_length(_: *const c_void, _: *const c_void) -> usize {
        c_int::MAX as usize + 1
    }
    export(
        "_ZNK2v86String10Utf8LengthEPNS_7IsolateE",
        large_length as *mut c_void,
    );
    export(
        "_ZNK2v86String9WriteUtf8EPNS_7IsolateEPcmiPm",
        write_modern as *mut c_void,
    );
    let symbols = Symbols::from_identifiers(&Identifiers::default());
    assert_eq!(
        unsafe { v8_sys::v8_string_utf8_length(&symbols, std::ptr::null(), std::ptr::null()) },
        c_int::MAX as usize + 1
    );
}
