#[cfg(any(target_os = "linux", target_os = "macos"))]
pub const V8_SCRIPT_COMPILER_COMPILE_FUNCTION_INTERNAL_SYMBOL: &str = "_ZN2v814ScriptCompiler23CompileFunctionInternalENS_5LocalINS_7ContextEEEPNS0_6SourceEmPNS1_INS_6StringEEEmPNS1_INS_6ObjectEEENS0_14CompileOptionsENS0_13NoCacheReasonEPNS1_INS_14ScriptOrModuleEEE";
#[cfg(target_os = "windows")]
pub const V8_SCRIPT_COMPILER_COMPILE_FUNCTION_INTERNAL_SYMBOL: &str = "?CompileFunctionInternal@ScriptCompiler@v8@@CA?AV?$MaybeLocal@VFunction@v8@@@2@V?$Local@VContext@v8@@@2@PEAVSource@12@_KQEAV?$Local@VString@v8@@@2@2QEAV?$Local@VObject@v8@@@2@W4CompileOptions@12@W4NoCacheReason@12@PEAV?$Local@VScriptOrModule@v8@@@2@@Z";

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub const V8_STRING_UTF8LENGTH_SYMBOL: &str = "_ZNK2v86String10Utf8LengthEPNS_7IsolateE";
#[cfg(target_os = "windows")]
pub const V8_STRING_UTF8LENGTH_SYMBOL: &str = "?Utf8Length@String@v8@@QEBAHPEAVIsolate@2@@Z";

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub const V8_STRING_WRITE_UTF8_SYMBOL: &str = "_ZNK2v86String9WriteUtf8EPNS_7IsolateEPciPii";
#[cfg(target_os = "windows")]
pub const V8_STRING_WRITE_UTF8_SYMBOL: &str = "?WriteUtf8@String@v8@@QEBAHPEAVIsolate@2@PEADHPEAHH@Z";

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub const V8_CONTEXT_GET_ISOLATE_SYMBOL: &str = "_ZN2v87Context10GetIsolateEv";
#[cfg(target_os = "windows")]
pub const V8_CONTEXT_GET_ISOLATE_SYMBOL: &str = "?GetIsolate@Context@v8@@QEAAPEAVIsolate@2@XZ";

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub const V8_STRING_NEW_FROM_UTF8_PTR: &str = "_ZN2v86String11NewFromUtf8EPNS_7IsolateEPKcNS_13NewStringTypeEi";
#[cfg(target_os = "windows")]
pub const V8_STRING_NEW_FROM_UTF8_PTR: &str = "?NewFromUtf8@String@v8@@SA?AV?$MaybeLocal@VString@v8@@@2@PEAVIsolate@2@PEBDW4NewStringType@2@H@Z";
