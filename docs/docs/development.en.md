# Development

## Build the Rust project

Install Rust **1.91.1** with [rustup](https://rustup.rs/), CMake 3.18 or newer, and the native C/C++ compiler and linker for your platform. CI uses Ubuntu 24.04, native Linux x64 and Windows x64 GNU cross-builds with MinGW-w64 POSIX and Wine. Native Windows/MSVC builds are unverified. macOS is experimental and has no CI tests.

From the repository root, build the workspace:

```sh
cargo build --locked --release
```

This builds the `crates/core` shared library and the `crates/launcher` executable into `target/release/`. CMake downloads and statically builds official [Dobby](https://github.com/jmpews/Dobby) revision `223aabced0431525c7d45196f9409fc505d58ac8`; the archive is verified with SHA-256 `b6e5054dd75c54dfe5a7b9585722af344984de8fbf0e8456b45260cbb0b8b73f`. The first build requires network access. No separate Dobby runtime library is needed.

The pinned revision supports the Windows backend. `crates/core/native/patch.cmake` supplies its missing Windows writable-page permission, adapts the x64 instrumentation bridge to the Windows calling convention and shadow space, and identifies Apple Silicon macOS correctly. Instrumentation is installed only at the configured function entry; it resumes the original instructions with the saved arguments and return convention. Supported entry ABIs are Linux/macOS x86-64, Linux/macOS AArch64, and Windows x64 MSVC V8 (including when the injector is built with GNU Rust).

Run the native ABI regression, which checks `CompileFunctionInternal`, `CompileFunction` and `CompileModule`, stack arguments, original execution and returned values:

```sh
cmake -S crates/core/native -B target/native-tests -DCMAKE_BUILD_TYPE=Release
cmake --build target/native-tests --config Release --target v8_killer_abi_test
target/native-tests/v8_killer_abi_test
```

With Visual Studio, run `target/native-tests/Release/v8_killer_abi_test.exe` instead.

## Node.js compatibility

Linux x86-64 debug/release injection has been verified with Node.js 22.23.2, 26.5.0 and 26.8.1 using default execution options, including the default type-stripping setting. Source processing covers CommonJS `.cjs`/`.js`, ESM `.mjs`, and `.js` under `"type":"module"`, including static imports, dynamic imports, ESM-to-CJS imports and synchronous CJS `require()` of ESM.

Node's [ModuleWrap](https://github.com/nodejs/node/blob/v26.8.1/src/module_wrap.cc) compiles ESM through `ScriptCompiler::CompileModule(Isolate*, Source*, CompileOptions, NoCacheReason)`. This entry uses the supplied isolate directly and shares source processing with the CJS hook. Both return `MaybeLocal`: Unix x86-64 passes isolate/source in RDI/RSI, AArch64 in X0/X1; Windows x64 MSVC uses a hidden return pointer in RCX and isolate/source in RDX/R8. The streamed `CompileModule` overload is not hooked.

`identifiers.V8_SCRIPT_COMPILER_COMPILE_MODULE` accepts the same symbol/RVA lists as existing identifiers. Omitting it uses built-in symbols, including in older custom identifier configurations; an empty list disables ESM hooking. A missing ESM symbol or failed ESM hook leaves the CJS hook available. ESM resource names are normally `file:` URLs, with non-ASCII filename characters percent-encoded; match the URL as reported by V8.

Node 26 removes `Context::GetIsolate`, `String::Utf8Length` and `String::WriteUtf8`. The fallback uses `Isolate::GetCurrent`, `Utf8LengthV2` and `WriteUtf8V2` with their own signatures (`size_t` lengths and the V2 flags/character-count argument order). Existing configured legacy identifiers retain priority. Missing required symbols still disable hooking.

The ABI was checked against Node [v26.8.1 V8 headers](https://github.com/nodejs/node/tree/v26.8.1/deps/v8/include) and [API implementation](https://github.com/nodejs/node/blob/v26.8.1/deps/v8/src/api/api.cc): `Source` still begins with source and resource-name `Local` fields; `Local`'s stored representation is also the public API receiver representation, with either direct or indirect handles. No heap-object offsets are used. Windows x64 V2 symbol declarations were checked with Clang's MSVC target. Future releases require the same exported signatures and source prefix layout; their compatibility is not guaranteed.

Run the UTF-8 regression against a built launcher and a chosen Node binary (Python 3 required):

```sh
python3 scripts/test-node.py target/debug/v8_killer_launcher /path/to/node
python3 scripts/test-node.py target/release/v8_killer_launcher /path/to/node
```

Each scenario compares direct Node execution, injection with a nonmatching rule, and injection with a matching rule. It asserts actual CJS/ESM replacements with Unicode/emoji/space paths and embedded NUL, imports, live bindings, top-level await, `import.meta`, unchanged unmatched modules, arguments and exit codes. The CI matrix below covers Node 22/24/26.

Official Node 22.23.2, 24.21.0 and 26.8.1 x64 binaries passed all 11 scenarios in debug/release on native Linux and under Wine with Windows GNU cross-built launcher/core binaries (396 executions total). Node 24.21.0 was selected from the official `latest-v24.x` manifest and SHA256-verified; its four combinations passed 132/132 executions with V8 13.6.233.17-node.53. Wine is not native Windows or an MSVC build. Use `--wine --file-output --report result.json` with Windows binaries; `winepath` converts target arguments. Pipe capture failed in the Wine baseline with `open EBADF`; regular-file capture passed. The local Wine 11 run required MinGW runtime DLLs on `WINEPATH`. Native Windows, macOS and non-default code-cache/streaming paths remain unverified.

## CI and caching

`.github/workflows/build.yaml` has two Ubuntu 24.04 jobs: `check (x86_64-unknown-linux-gnu)` and `check (x86_64-pc-windows-gnu)`. PRs build debug once per target, then test official Node **22.23.2 / 24.21.0 / 26.8.1** sequentially: 6 combinations × 11 scenarios × 3 controls = **198 executions**. `master` pushes and `workflow_dispatch` run debug and release: 12 combinations, **396 executions**. Unit tests, native ABI regression and Clippy run in these jobs; Linux also checks rustfmt. Clippy uses `-- -D warnings` without changing `RUSTFLAGS`; Cargo commands use `--locked`. Windows tests use Wine and file output, not native Windows/MSVC.

Feature branches trigger through PRs only; `master` has push coverage. Older runs of the same PR/ref are cancelled. Cargo manifests, crates, scripts, Cargo/toolchain configuration and CI workflows trigger code checks; documentation-only changes use the docs workflow. Run the CI workflow manually to get the full matrix and release artifacts. PRs upload only JSON regression reports, retained for 7 days. No automatic macOS builds, tests or Clippy runs are configured.

Cache behavior:

- `Swatinem/rust-cache@v2` caches Cargo registry/git and `target`, with `cache-workspace-crates: true` so core's build-script `OUT_DIR` and CMake outputs survive cleanup. The action keeps selected packages' `build`, `.fingerprint` and `deps`, but removes top-level binaries and incremental output; Cargo reconstructs those as needed. Exact cache hits are immutable, not overwritten on every run.
- Keys separate Ubuntu label, target, profile set, Rust version/environment, compiler/CMake/package versions and native sources/build script/workflow. The action adds Cargo manifest/lockfile hashes and may restore a prior dependency cache within the same native/toolchain boundary. Cargo handles Rust source changes; native changes cannot restore incompatible CMake output. Debug/release keep their separate Cargo `OUT_DIR`s even though the native library itself uses Release optimization.
- Standalone ABI builds live in `.cache/native-tests`, covered by the same native-aware Rust cache key as an additional directory. They are outside `target` to avoid Rust-cache cleanup and duplicate target caching. Only this standalone build uses that path.
- The Dobby archive is shared by its pinned SHA256 and rechecked before use; `V8_KILLER_DOBBY_ARCHIVE` supplies it to CMake, which also verifies its SHA256. Extracted/patched sources and compiled objects stay in each CMake build tree. Official Node downloads/extractions are cached by exact versions and target; each use verifies the archive/executable against the official version-specific `SHASUMS256.txt`, and Linux extraction is refreshed from the verified archive.
- Wine prefixes are never cached. MinGW/Wine installation is centralized in the job, with tool/package versions logged. There is no remote compiler cache or custom image.

These workflows have **not yet run remotely**. Local functional tests do not prove GitHub cache hits or runner compatibility. The first remote PR/full-matrix runs must verify both cold and warm cache behavior and timings. Consolidation removes the old `build`, `test`, `node`, `clippy_check` and `rustfmt` check contexts: inspect branch protection/rulesets for required checks before merging. Remote branch settings have not been changed; path-filtered workflows may remain pending if required for documentation-only PRs.

## Electron string ABI compatibility

Electron 44.4.3 uses V8 15.2.124.28-electron.0. Its Linux x64 and macOS x64/arm64 exports use the renamed `WriteUtf8(Isolate*, char*, size_t, int, size_t*)`, returning `size_t`; Windows x64 retains the V2 exports. Both names use the size_t branch, with existing legacy identifiers taking priority and V2 preceding the renamed fallback. The [V8 header](https://github.com/v8/v8/blob/15.2.124.28/include/v8-primitive.h) explicitly forwards `WriteUtf8V2` to `WriteUtf8`. The [API implementation](https://github.com/v8/v8/blob/15.2.124.28/src/api/api.cc) and [encoder wrapper](https://github.com/v8/v8/blob/15.2.124.28/src/objects/string.cc) confirm byte capacity/return values, optional processed input character counts, and flags `kNullTerminate=1`, `kReplaceInvalidUtf8=2`. We use flag 2 without a terminator and preserve embedded NUL bytes.

The renamed `Utf8Length` also returns `size_t`. Its Itanium name does not encode the return type, so the renamed writer identifies this API generation; a configured length address matching that export is called with the size_t ABI. Other configured legacy symbol/RVA addresses retain priority. `V8_STRING_WRITE_UTF8` remains a legacy ABI configuration field: do not put the new signature there. `NewFromUtf8` still accepts an `int` byte length. Missing required symbols still disable hooking.

Run the standalone fake-function regression (no core library constructor, native symbol lookup or hooks):

```sh
cargo test --locked -p v8_killer_core --test string_abi
```

Symbol/ABI adaptation and pure Rust checks do **not** establish Electron injection support. Real Electron injection has not been verified. The separate clean Electron 28.3.3/44.4.3 application baseline only verifies ordinary module loading; Windows/macOS observations are static export checks.

## Develop the documentation

The documentation uses MkDocs and [uv](https://docs.astral.sh/uv/getting-started/installation/). Python 3.14.7 is pinned in `docs/.python-version`; uv manages Python and an isolated environment for the documentation.

From the repository root:

```sh
cd docs
uv python install
uv sync --locked
```

Preview both languages locally:

```sh
uv run --locked mkdocs serve
```

Open the address printed by MkDocs and use the language selector to switch between English and Chinese. Edit the `.en.md` and `.zh.md` files in `docs/docs/` together.

Build with the same strict validation used in CI:

```sh
uv run --locked mkdocs build --strict
```

The generated site is written to `docs/site/` relative to the repository root. Commit dependency changes in `docs/pyproject.toml` together with the `docs/uv.lock` generated by `uv lock`.
