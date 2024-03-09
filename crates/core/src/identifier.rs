use frida_gum::{Module, NativePointer};
use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub(crate) struct Identifiers {
    pub(crate) V8_SCRIPT_COMPILER_COMPILE_FUNCTION_INTERNAL: Vec<IdentifierEnum>,
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
        self.iter().find_map(|identifier| identifier.identify())
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
            let ptr = Module::find_export_by_name(None, symbol);
            if ptr.is_some() {
                return ptr;
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
        let base_address = Module::find_base_address(&self.module_name);
        if base_address.is_null() {
            return None;
        }
        Some(NativePointer(unsafe { base_address.0.add(self.rva) }))
    }
}

impl Default for Identifiers {
    fn default() -> Self {
        Identifiers {
            V8_SCRIPT_COMPILER_COMPILE_FUNCTION_INTERNAL: vec![
                IdentifierEnum::SymbolIdentifier(SymbolIdentifier {
                    symbols: vec![
                        "_ZN2v814ScriptCompiler23CompileFunctionInternalENS_5LocalINS_7ContextEEEPNS0_6SourceEmPNS1_INS_6StringEEEmPNS1_INS_6ObjectEEENS0_14CompileOptionsENS0_13NoCacheReasonEPNS1_INS_14ScriptOrModuleEEE"
                            .to_string(),
                        "?CompileFunctionInternal@ScriptCompiler@v8@@CA?AV?$MaybeLocal@VFunction@v8@@@2@V?$Local@VContext@v8@@@2@PEAVSource@12@_KQEAV?$Local@VString@v8@@@2@2QEAV?$Local@VObject@v8@@@2@W4CompileOptions@12@W4NoCacheReason@12@PEAV?$Local@VScriptOrModule@v8@@@2@@Z"
                            .to_string(),
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
