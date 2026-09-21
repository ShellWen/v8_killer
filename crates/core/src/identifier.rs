use serde::Deserialize;
use std::ffi::{c_char, c_void, CString};
use tracing::debug;

#[derive(Clone, Copy, Debug)]
pub(crate) struct NativePointer(pub(crate) *mut c_void);

unsafe extern "C" {
    fn v8_killer_symbol(name: *const c_char) -> *mut c_void;
    fn v8_killer_module(name: *const c_char) -> *mut c_void;
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub(crate) struct Symbols {
    pub(crate) V8_SCRIPT_COMPILER_COMPILE_FUNCTION: Option<NativePointer>,
    pub(crate) V8_SCRIPT_COMPILER_COMPILE_MODULE: Option<NativePointer>,
    pub(crate) V8_STRING_UTF8LENGTH: Option<NativePointer>,
    pub(crate) V8_STRING_WRITE_UTF8: Option<NativePointer>,
    pub(crate) V8_CONTEXT_GET_ISOLATE: Option<NativePointer>,
    pub(crate) V8_STRING_NEW_FROM_UTF8: Option<NativePointer>,
    pub(crate) V8_STRING_UTF8LENGTH_SIZE_T: Option<NativePointer>,
    pub(crate) V8_STRING_WRITE_UTF8_SIZE_T: Option<NativePointer>,
    pub(crate) V8_ISOLATE_GET_CURRENT: Option<NativePointer>,
}

// Bypass check for `Send` and `Sync` traits
unsafe impl Sync for Symbols {}
unsafe impl Send for Symbols {}

impl Symbols {
    pub(crate) fn is_complete(&self) -> bool {
        [
            self.V8_SCRIPT_COMPILER_COMPILE_FUNCTION,
            self.V8_STRING_UTF8LENGTH
                .or(self.V8_STRING_UTF8LENGTH_SIZE_T),
            self.V8_STRING_WRITE_UTF8
                .or(self.V8_STRING_WRITE_UTF8_SIZE_T),
            self.V8_CONTEXT_GET_ISOLATE.or(self.V8_ISOLATE_GET_CURRENT),
            self.V8_STRING_NEW_FROM_UTF8,
        ]
        .iter()
        .all(Option::is_some)
    }

    pub(crate) fn from_identifiers(identifiers: &Identifiers) -> Self {
        let renamed_writer = SymbolIdentifier {
            symbols: vec![
                "_ZNK2v86String9WriteUtf8EPNS_7IsolateEPcmiPm".into(),
                "?WriteUtf8@String@v8@@QEBA_KPEAVIsolate@2@PEAD_KHPEA_K@Z".into(),
            ],
        }
        .identify();
        let renamed_length = renamed_writer.and_then(|_| {
            SymbolIdentifier {
                symbols: vec![
                    "_ZNK2v86String10Utf8LengthEPNS_7IsolateE".into(),
                    "?Utf8Length@String@v8@@QEBA_KPEAVIsolate@2@@Z".into(),
                ],
            }
            .identify()
        });
        let configured_length = identifiers.V8_STRING_UTF8LENGTH.identify();
        // Itanium omits return types; the renamed writer identifies the size_t API.
        let legacy_length = configured_length
            .filter(|pointer| renamed_length.is_none_or(|modern| modern.0 != pointer.0));
        Symbols {
            V8_SCRIPT_COMPILER_COMPILE_FUNCTION: identifiers
                .V8_SCRIPT_COMPILER_COMPILE_FUNCTION
                .identify(),
            V8_SCRIPT_COMPILER_COMPILE_MODULE: identifiers
                .V8_SCRIPT_COMPILER_COMPILE_MODULE
                .identify(),
            V8_STRING_UTF8LENGTH: legacy_length,
            V8_STRING_WRITE_UTF8: identifiers.V8_STRING_WRITE_UTF8.identify(),
            V8_CONTEXT_GET_ISOLATE: identifiers.V8_CONTEXT_GET_ISOLATE.identify(),
            V8_STRING_NEW_FROM_UTF8: identifiers.V8_STRING_NEW_FROM_UTF8.identify(),
            V8_STRING_UTF8LENGTH_SIZE_T: SymbolIdentifier {
                symbols: vec![
                    "_ZNK2v86String12Utf8LengthV2EPNS_7IsolateE".into(),
                    "?Utf8LengthV2@String@v8@@QEBA_KPEAVIsolate@2@@Z".into(),
                ],
            }
            .identify()
            .or(renamed_length),
            V8_STRING_WRITE_UTF8_SIZE_T: SymbolIdentifier {
                symbols: vec![
                    "_ZNK2v86String11WriteUtf8V2EPNS_7IsolateEPcmiPm".into(),
                    "?WriteUtf8V2@String@v8@@QEBA_KPEAVIsolate@2@PEAD_KHPEA_K@Z".into(),
                ],
            }
            .identify()
            .or(renamed_writer),
            V8_ISOLATE_GET_CURRENT: SymbolIdentifier {
                symbols: vec![
                    "_ZN2v87Isolate10GetCurrentEv".into(),
                    "?GetCurrent@Isolate@v8@@SAPEAV12@XZ".into(),
                ],
            }
            .identify(),
        }
    }
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub(crate) struct Identifiers {
    pub(crate) V8_SCRIPT_COMPILER_COMPILE_FUNCTION: Vec<IdentifierEnum>,
    #[serde(default = "module_identifiers")]
    pub(crate) V8_SCRIPT_COMPILER_COMPILE_MODULE: Vec<IdentifierEnum>,
    pub(crate) V8_STRING_UTF8LENGTH: Vec<IdentifierEnum>,
    pub(crate) V8_STRING_WRITE_UTF8: Vec<IdentifierEnum>,
    pub(crate) V8_CONTEXT_GET_ISOLATE: Vec<IdentifierEnum>,
    pub(crate) V8_STRING_NEW_FROM_UTF8: Vec<IdentifierEnum>,
}

pub(crate) trait Identifier {
    fn identify(&self) -> Option<NativePointer>;
}

impl Identifier for Vec<IdentifierEnum> {
    fn identify(&self) -> Option<NativePointer> {
        self.iter()
            .find_map(|identifier| match identifier.identify() {
                Some(ptr) => {
                    debug!("Identifier found: {:?}, by {:?}", ptr.0, identifier);
                    Some(ptr)
                }
                None => None,
            })
    }
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub(crate) enum IdentifierEnum {
    #[serde(rename = "symbol")]
    SymbolIdentifier(SymbolIdentifier),
    #[serde(rename = "rva")]
    RvaIdentifier(RvaIdentifier),
}

impl Identifier for IdentifierEnum {
    fn identify(&self) -> Option<NativePointer> {
        match self {
            IdentifierEnum::SymbolIdentifier(identifier) => identifier.identify(),
            IdentifierEnum::RvaIdentifier(identifier) => identifier.identify(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct SymbolIdentifier {
    pub(crate) symbols: Vec<String>,
}

impl Identifier for SymbolIdentifier {
    fn identify(&self) -> Option<NativePointer> {
        for symbol in &self.symbols {
            let Ok(symbol) = CString::new(symbol.as_str()) else {
                continue;
            };
            let ptr = unsafe { v8_killer_symbol(symbol.as_ptr()) };
            if !ptr.is_null() {
                return Some(NativePointer(ptr));
            }
        }
        None
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct RvaIdentifier {
    pub(crate) module_name: String,
    pub(crate) rva: usize,
}

impl Identifier for RvaIdentifier {
    fn identify(&self) -> Option<NativePointer> {
        let name = CString::new(self.module_name.as_str()).ok()?;
        let base = unsafe { v8_killer_module(name.as_ptr()) };
        if base.is_null() {
            return None;
        }
        (base as usize)
            .checked_add(self.rva)
            .map(|address| NativePointer(address as *mut c_void))
    }
}

fn module_identifiers() -> Vec<IdentifierEnum> {
    vec![IdentifierEnum::SymbolIdentifier(SymbolIdentifier {
        symbols: vec![
            "_ZN2v814ScriptCompiler13CompileModuleEPNS_7IsolateEPNS0_6SourceENS0_14CompileOptionsENS0_13NoCacheReasonE".into(),
            "?CompileModule@ScriptCompiler@v8@@SA?AV?$MaybeLocal@VModule@v8@@@2@PEAVIsolate@2@PEAVSource@12@W4CompileOptions@12@W4NoCacheReason@12@@Z".into(),
        ],
    })]
}

impl Default for Identifiers {
    fn default() -> Self {
        Identifiers {
            V8_SCRIPT_COMPILER_COMPILE_MODULE: module_identifiers(),
            V8_SCRIPT_COMPILER_COMPILE_FUNCTION: vec![
                IdentifierEnum::SymbolIdentifier(SymbolIdentifier {
                    symbols: vec![
                        "_ZN2v814ScriptCompiler23CompileFunctionInternalENS_5LocalINS_7ContextEEEPNS0_6SourceEmPNS1_INS_6StringEEEmPNS1_INS_6ObjectEEENS0_14CompileOptionsENS0_13NoCacheReasonEPNS1_INS_14ScriptOrModuleEEE"
                            .to_string(),
                        "?CompileFunctionInternal@ScriptCompiler@v8@@CA?AV?$MaybeLocal@VFunction@v8@@@2@V?$Local@VContext@v8@@@2@PEAVSource@12@_KQEAV?$Local@VString@v8@@@2@2QEAV?$Local@VObject@v8@@@2@W4CompileOptions@12@W4NoCacheReason@12@PEAV?$Local@VScriptOrModule@v8@@@2@@Z"
                            .to_string(),
                        // fallback for newer v8 versions
                        "_ZN2v814ScriptCompiler15CompileFunctionENS_5LocalINS_7ContextEEEPNS0_6SourceEmPNS1_INS_6StringEEEmPNS1_INS_6ObjectEEENS0_14CompileOptionsENS0_13NoCacheReasonE".to_string(),
                        "?CompileFunction@ScriptCompiler@v8@@SA?AV?$MaybeLocal@VFunction@v8@@@2@V?$Local@VContext@v8@@@2@PEAVSource@12@_KQEAV?$Local@VString@v8@@@2@2QEAV?$Local@VObject@v8@@@2@W4CompileOptions@12@W4NoCacheReason@12@@Z".to_string(),
                    ],
                })
            ],
            V8_STRING_UTF8LENGTH: vec![
                IdentifierEnum::SymbolIdentifier(SymbolIdentifier {
                    symbols: vec![
                        "_ZNK2v86String10Utf8LengthEPNS_7IsolateE".to_string(),
                        "?Utf8Length@String@v8@@QEBAHPEAVIsolate@2@@Z".to_string(),
                    ],
                }
                )],
            V8_STRING_WRITE_UTF8: vec![
                IdentifierEnum::SymbolIdentifier(SymbolIdentifier {
                    symbols: vec![
                        "_ZNK2v86String9WriteUtf8EPNS_7IsolateEPciPii".to_string(),
                        "?WriteUtf8@String@v8@@QEBAHPEAVIsolate@2@PEADHPEAHH@Z".to_string(),
                    ],
                }
                )],
            V8_CONTEXT_GET_ISOLATE: vec![
                IdentifierEnum::SymbolIdentifier(SymbolIdentifier {
                    symbols: vec![
                        "_ZN2v87Context10GetIsolateEv".to_string(),
                        "?GetIsolate@Context@v8@@QEAAPEAVIsolate@2@XZ".to_string(),
                    ],
                }
                )],
            V8_STRING_NEW_FROM_UTF8: vec![
                IdentifierEnum::SymbolIdentifier(SymbolIdentifier {
                    symbols: vec![
                        "_ZN2v86String11NewFromUtf8EPNS_7IsolateEPKcNS_13NewStringTypeEi".to_string(),
                        "?NewFromUtf8@String@v8@@SA?AV?$MaybeLocal@VString@v8@@@2@PEAVIsolate@2@PEBDW4NewStringType@2@H@Z".to_string(),
                    ],
                }
                )],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_config_defaults_module_identifiers_and_accepts_overrides() {
        let legacy = "V8_SCRIPT_COMPILER_COMPILE_FUNCTION = []\nV8_STRING_UTF8LENGTH = []\nV8_STRING_WRITE_UTF8 = []\nV8_CONTEXT_GET_ISOLATE = []\nV8_STRING_NEW_FROM_UTF8 = []\n";
        let identifiers: Identifiers = toml::from_str(legacy).unwrap();
        assert_eq!(identifiers.V8_SCRIPT_COMPILER_COMPILE_MODULE.len(), 1);
        let identifiers: Identifiers =
            toml::from_str(&format!("{legacy}V8_SCRIPT_COMPILER_COMPILE_MODULE = []\n")).unwrap();
        assert!(identifiers.V8_SCRIPT_COMPILER_COMPILE_MODULE.is_empty());
    }

    #[test]
    fn missing_identifiers_return_none() {
        assert!(SymbolIdentifier {
            symbols: vec!["v8_killer_missing_symbol".into(), "invalid\0name".into()]
        }
        .identify()
        .is_none());
        assert!(RvaIdentifier {
            module_name: "v8_killer_missing_module".into(),
            rva: 0
        }
        .identify()
        .is_none());
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn rva_resolves_the_main_executable_by_path_and_name() {
        let path = std::env::current_exe().unwrap();
        let resolve = |name: &str, rva| {
            RvaIdentifier {
                module_name: name.into(),
                rva,
            }
            .identify()
            .unwrap()
            .0 as usize
        };
        let base = resolve(path.to_str().unwrap(), 0);
        assert_ne!(base, 0);
        assert_eq!(
            resolve(path.file_name().unwrap().to_str().unwrap(), 17),
            base + 17
        );
    }

    #[test]
    fn incomplete_symbols_disable_hooking() {
        let pointer = Some(NativePointer(std::ptr::dangling_mut::<u8>().cast()));
        for missing in 0..=5 {
            let mut pointers = [pointer; 5];
            if missing < pointers.len() {
                pointers[missing] = None;
            }
            let symbols = Symbols {
                V8_SCRIPT_COMPILER_COMPILE_FUNCTION: pointers[0],
                V8_SCRIPT_COMPILER_COMPILE_MODULE: None,
                V8_STRING_UTF8LENGTH: pointers[1],
                V8_STRING_WRITE_UTF8: pointers[2],
                V8_CONTEXT_GET_ISOLATE: pointers[3],
                V8_STRING_NEW_FROM_UTF8: pointers[4],
                V8_STRING_UTF8LENGTH_SIZE_T: None,
                V8_STRING_WRITE_UTF8_SIZE_T: None,
                V8_ISOLATE_GET_CURRENT: None,
            };
            assert_eq!(symbols.is_complete(), missing == 5);
        }
    }

    #[test]
    fn modern_symbols_require_both_string_apis_and_isolate() {
        let pointer = Some(NativePointer(std::ptr::dangling_mut::<u8>().cast()));
        for missing in 0..=3 {
            let mut pointers = [pointer; 3];
            if missing < pointers.len() {
                pointers[missing] = None;
            }
            let symbols = Symbols {
                V8_SCRIPT_COMPILER_COMPILE_FUNCTION: pointer,
                V8_SCRIPT_COMPILER_COMPILE_MODULE: None,
                V8_STRING_UTF8LENGTH: None,
                V8_STRING_WRITE_UTF8: None,
                V8_CONTEXT_GET_ISOLATE: None,
                V8_STRING_NEW_FROM_UTF8: pointer,
                V8_STRING_UTF8LENGTH_SIZE_T: pointers[0],
                V8_STRING_WRITE_UTF8_SIZE_T: pointers[1],
                V8_ISOLATE_GET_CURRENT: pointers[2],
            };
            assert_eq!(symbols.is_complete(), missing == 3);
        }
    }
}
