# 开发

## 构建 Rust 项目

通过 [rustup](https://rustup.rs/) 安装 Rust **1.91.1**，以及当前平台的原生编译器和链接器。CI 使用 Ubuntu 24.04，覆盖原生 Linux x64，以及 MinGW-w64 POSIX 交叉构建并通过 Wine 运行的 Windows x64 GNU。原生 Windows/MSVC 构建尚未验证。macOS 为实验性支持，没有 CI 测试。

在仓库根目录构建工作区：

```sh
cargo build --locked --release
```

这会将 `crates/core` 共享库和 `crates/launcher` 可执行文件构建到 `target/release/`。额外需要 CMake 3.18 或更新版本及平台 C/C++ 工具链。CMake 下载并静态构建官方 [Dobby](https://github.com/jmpews/Dobby) 固定版本 `223aabced0431525c7d45196f9409fc505d58ac8`，使用 SHA-256 `b6e5054dd75c54dfe5a7b9585722af344984de8fbf0e8456b45260cbb0b8b73f` 校验源码归档。首次构建需要联网，运行时无需额外 Dobby 动态库。

该固定版本保留 Windows 后端。`crates/core/native/patch.cmake` 补充 Windows 可写页权限、适配 x64 instrumentation 调用桥的 Windows 参数寄存器和 shadow space，并修正 Apple Silicon macOS 平台识别。仅在配置的函数入口安装 instrumentation，随后恢复原参数和返回约定，继续执行原指令。支持的入口 ABI 为 Linux/macOS x86-64、Linux/macOS AArch64，以及 Windows x64 MSVC V8（包括使用 GNU Rust 构建注入器的情况）。

原生 ABI 回归检查 `CompileFunctionInternal`、`CompileFunction` 和 `CompileModule` 的参数、原函数执行及返回值：

```sh
cmake -S crates/core/native -B target/native-tests -DCMAKE_BUILD_TYPE=Release
cmake --build target/native-tests --config Release --target v8_killer_abi_test
target/native-tests/v8_killer_abi_test
```

使用 Visual Studio 时运行 `target/native-tests/Release/v8_killer_abi_test.exe`。

## Node.js 兼容性

Linux x86-64 debug/release 注入已使用默认执行选项验证 Node.js 22.23.2、26.5.0 和 26.8.1，包含默认的类型剥离设置。源码处理覆盖 CommonJS `.cjs`/`.js`、ESM `.mjs` 和 `"type":"module"` 下的 `.js`，包括静态导入、动态导入、ESM 导入 CJS 和 CJS 同步 `require()` ESM。

Node 的 [ModuleWrap](https://github.com/nodejs/node/blob/v26.8.1/src/module_wrap.cc) 通过 `ScriptCompiler::CompileModule(Isolate*, Source*, CompileOptions, NoCacheReason)` 编译 ESM。此入口直接使用传入的 isolate，与 CJS hook 共享源码处理。两者均返回 `MaybeLocal`：Unix x86-64 使用 RDI/RSI 传递 isolate/source，AArch64 使用 X0/X1；Windows x64 MSVC 的 RCX 是隐藏返回指针，isolate/source 位于 RDX/R8。未挂钩流式 `CompileModule` 重载。

`identifiers.V8_SCRIPT_COMPILER_COMPILE_MODULE` 接受与现有标识符相同的 symbol/RVA 列表。省略该字段时使用内置符号，旧自定义标识符配置亦兼容；空列表禁用 ESM hook。缺少 ESM 符号或 ESM hook 安装失败不影响 CJS hook。ESM 资源名通常是 `file:` URL，文件名中的非 ASCII 字符会被百分号编码；匹配规则应使用 V8 实际提供的 URL。

Node 26 移除了 `Context::GetIsolate`、`String::Utf8Length` 和 `String::WriteUtf8`。兼容分支使用 `Isolate::GetCurrent`、`Utf8LengthV2` 和 `WriteUtf8V2`，分别按其真实签名调用，包括 `size_t` 长度及 V2 的 flags/字符数参数顺序。已配置的旧版标识符保留优先级，缺少必要符号时仍禁用 hook。

ABI 依据 Node [v26.8.1 V8 头文件](https://github.com/nodejs/node/tree/v26.8.1/deps/v8/include)和 [API 实现](https://github.com/nodejs/node/blob/v26.8.1/deps/v8/src/api/api.cc)核对：`Source` 开头仍为源码、资源名两个 `Local` 字段；无论 direct 还是 indirect handle，`Local` 存储表示均与公开 API 的 receiver 表示一致，无须读取堆对象偏移。Windows x64 V2 符号声明已通过 Clang MSVC target 核对。后续版本需保持相同导出签名和 Source 前缀布局，不保证未来版本兼容。

使用已构建的 launcher 和指定 Node 运行 UTF-8 回归检查（需要 Python 3）：

```sh
python3 scripts/test-node.py target/debug/v8_killer_launcher /path/to/node
python3 scripts/test-node.py target/release/v8_killer_launcher /path/to/node
```

每个场景对比直接 Node、不匹配规则注入、匹配规则注入三次执行，断言 CJS/ESM 中 Unicode/emoji/空格路径和内嵌 NUL 的真实替换，同时验证导入、live binding、顶层 await、`import.meta`、未匹配模块、参数及退出码。下方 CI 矩阵覆盖 Node 22/24/26。

官方 Node 22.23.2、24.21.0 和 26.8.1 x64 在原生 Linux 及 Wine 下的 Windows GNU 交叉构建 launcher/core 上，debug/release 均通过全部 11 个场景（合计 396 次执行）。Node 24.21.0 取自官方 `latest-v24.x` 清单并校验 SHA256，V8 为 13.6.233.17-node.53，四个组合通过 132/132 次执行。Wine 结果不代表原生 Windows 或 MSVC 构建。Windows 二进制使用 `--wine --file-output --report result.json`，通过 `winepath` 转换目标参数。Wine 管道采集基线报 `open EBADF`，普通文件采集通过。本地 Wine 11 运行需通过 `WINEPATH` 提供 MinGW runtime DLL。原生 Windows、macOS 和非默认代码缓存/流式编译路径仍未验证。

## CI 与缓存

`.github/workflows/build.yaml` 包含两个 Ubuntu 24.04 job：`check (x86_64-unknown-linux-gnu)` 与 `check (x86_64-pc-windows-gnu)`。PR 每个 target 只构建一次 debug，随后依次测试官方 Node **22.23.2 / 24.21.0 / 26.8.1**：6 组合 × 11 场景 × 3 对照 = **198 次执行**。`master` push 和 `workflow_dispatch` 运行 debug 与 release：12 组合，共 **396 次执行**。单元测试、原生 ABI 回归及 Clippy 合并在这些 job 内；Linux 额外检查 rustfmt。Clippy 使用 `-- -D warnings`，不改变 `RUSTFLAGS`；Cargo 命令使用 `--locked`。Windows 测试使用 Wine 与文件输出，不是原生 Windows/MSVC。

功能分支只通过 PR 触发，`master` 通过 push 触发；同一 PR/ref 的旧运行自动取消。Cargo 清单、crates、scripts、Cargo/工具链配置及 CI 工作流变更触发代码检查；仅文档变更运行 docs workflow。手动运行 CI 可执行完整矩阵并获取 release 产物。PR 仅上传 JSON 回归报告，保留 7 天。不自动构建、测试 macOS 或运行 macOS Clippy。

缓存机制：

- `Swatinem/rust-cache@v2` 缓存 Cargo registry/git 与 `target`，启用 `cache-workspace-crates: true`，保留 core build-script 的 `OUT_DIR` 和 CMake 输出。action 清理时保留所选包的 `build`、`.fingerprint`、`deps`，移除顶层二进制及增量目录，由 Cargo 按需恢复。精确命中的缓存不可变，不会每次覆盖。
- key 隔离 Ubuntu 标签、target、profile 集合、Rust 版本/环境、编译器/CMake/软件包版本及原生源码/build script/workflow。action 追加 Cargo 清单/锁文件哈希，可在相同原生/工具链边界内恢复旧依赖缓存。Rust 源码变更由 Cargo 判断；原生变更不会恢复不兼容的 CMake 输出。即使原生库本身采用 Release 优化，debug/release 仍使用各自 Cargo `OUT_DIR`。
- 独立 ABI 构建位于 `.cache/native-tests`，作为附加目录使用同一带原生指纹的 Rust 缓存 key，避开 target 清理且不重复缓存整个 target。该路径只用于独立 ABI 构建。
- Dobby 源码归档按固定 SHA256 共享，每次使用前重新校验，通过 `V8_KILLER_DOBBY_ARCHIVE` 提供给 CMake，CMake 再次校验 SHA256。解压/补丁源码及编译对象保留在各自 CMake 构建树内。官方 Node 下载/解压目录按精确版本与 target 缓存；每次使用按官方版本对应的 `SHASUMS256.txt` 校验归档或可执行文件，Linux 从已校验归档刷新解压结果。
- 不缓存 Wine prefix。MinGW/Wine 在 job 中统一安装并记录工具/软件包版本，没有远程编译缓存或自建镜像。

这些工作流**尚未远程运行**。本地功能验证不能证明 GitHub 缓存命中或 runner 兼容；首次远程 PR/完整矩阵运行仍需核对冷/热缓存与耗时。合并工作流会移除原有 `build`、`test`、`node`、`clippy_check`、`rustfmt` check contexts，合并前需检查 branch protection/rulesets 中的 required checks。未修改远程分支设置；若路径过滤的工作流被设为 required，仅文档 PR 可能一直 pending。

## Electron 字符串 ABI 兼容性

Electron 44.4.3 使用 V8 15.2.124.28-electron.0。Linux x64 和 macOS x64/arm64 导出重命名后的 `WriteUtf8(Isolate*, char*, size_t, int, size_t*)`，返回 `size_t`；Windows x64 保留 V2 导出。两个名称共用 size_t 分支，现有旧版标识符优先，随后依次尝试 V2 和新名称。[V8 头文件](https://github.com/v8/v8/blob/15.2.124.28/include/v8-primitive.h)明确将 `WriteUtf8V2` 转发到 `WriteUtf8`。[API 实现](https://github.com/v8/v8/blob/15.2.124.28/src/api/api.cc)与[编码包装实现](https://github.com/v8/v8/blob/15.2.124.28/src/objects/string.cc)确认容量和返回值按字节计数，可选输出为已处理输入字符数，flags 为 `kNullTerminate=1`、`kReplaceInvalidUtf8=2`。当前使用 flag 2，不追加终止符，保留内嵌 NUL 字节。

重命名后的 `Utf8Length` 也返回 `size_t`。Itanium 名称不编码返回类型，因此通过新 writer 识别此 API 代际；配置的长度地址若匹配该导出，则按 size_t ABI 调用。其他自定义旧 ABI symbol/RVA 地址仍优先。`V8_STRING_WRITE_UTF8` 配置字段仍表示旧 ABI，不能填入新签名。`NewFromUtf8` 的字节长度仍为 `int`。缺少必要符号时仍禁用 hook。

运行独立 fake 函数回归（不执行 core 库构造函数、原生符号查找或 hook）：

```sh
cargo test --locked -p v8_killer_core --test string_abi
```

符号/ABI 适配与纯 Rust 检查**不代表** Electron 注入支持已验证。尚未验证真实 Electron 注入。独立的 Electron 28.3.3/44.4.3 普通应用基线仅验证正常模块加载；Windows/macOS 结果仅为静态导出检查。

## 开发文档

文档使用 MkDocs 和 [uv](https://docs.astral.sh/uv/getting-started/installation/)。`docs/.python-version` 固定 Python 3.14.7，由 uv 管理 Python 和文档的隔离环境。

从仓库根目录执行：

```sh
cd docs
uv python install
uv sync --locked
```

在本地预览中英文文档：

```sh
uv run --locked mkdocs serve
```

打开 MkDocs 输出的地址，通过语言选择器切换中英文。请同步编辑 `docs/docs/` 中的 `.en.md` 和 `.zh.md` 文件。

使用与 CI 相同的严格检查构建文档：

```sh
uv run --locked mkdocs build --strict
```

生成的站点位于仓库根目录下的 `docs/site/`。修改依赖时，请一并提交 `docs/pyproject.toml` 和通过 `uv lock` 生成的 `docs/uv.lock`。
