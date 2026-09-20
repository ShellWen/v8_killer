# 开发

## 构建 Rust 项目

通过 [rustup](https://rustup.rs/) 安装稳定版 Rust，并安装当前平台的原生编译器和链接器。Windows 使用 MSVC 工具链及 Visual Studio C++ 构建工具。

在仓库根目录构建工作区：

```sh
cargo build --locked --release
```

这会将 `crates/core` 共享库和 `crates/launcher` 可执行文件构建到 `target/release/`。额外需要 CMake 3.18 或更新版本及平台 C/C++ 工具链。CMake 下载并静态构建官方 [Dobby](https://github.com/jmpews/Dobby) 固定版本 `223aabced0431525c7d45196f9409fc505d58ac8`，使用 SHA-256 `b6e5054dd75c54dfe5a7b9585722af344984de8fbf0e8456b45260cbb0b8b73f` 校验源码归档。首次构建需要联网，运行时无需额外 Dobby 动态库。

该固定版本保留 Windows 后端。`crates/core/native/patch.cmake` 补充 Windows 可写页权限、适配 x64 instrumentation 调用桥的 Windows 参数寄存器和 shadow space，并修正 Apple Silicon macOS 平台识别。仅在配置的函数入口安装 instrumentation，随后恢复原参数和返回约定，继续执行原指令。支持的入口 ABI 为 Linux/macOS x86-64、Linux/macOS AArch64，以及 Windows x64 MSVC V8（包括使用 GNU Rust 构建注入器的情况）。

原生 ABI 回归检查两种编译入口的栈参数、原函数执行及返回值：

```sh
cmake -S crates/core/native -B target/native-tests -DCMAKE_BUILD_TYPE=Release
cmake --build target/native-tests --config Release --target v8_killer_abi_test
target/native-tests/v8_killer_abi_test
```

使用 Visual Studio 时运行 `target/native-tests/Release/v8_killer_abi_test.exe`。

## Node.js 兼容性

Linux x86-64 注入已验证 Node.js 22.23.2、26.5.0 和 26.8.1。CommonJS `.cjs`、`.js` 文件使用 Node 默认执行选项即可处理，包含默认的类型剥离设置。ESM 导入的 CommonJS 可以处理；ESM 源码本身使用 `CompileModule`，不在当前 `CompileFunction` hook 范围内。

Node 26 移除了 `Context::GetIsolate`、`String::Utf8Length` 和 `String::WriteUtf8`。兼容分支使用 `Isolate::GetCurrent`、`Utf8LengthV2` 和 `WriteUtf8V2`，分别按其真实签名调用，包括 `size_t` 长度及 V2 的 flags/字符数参数顺序。已配置的旧版标识符保留优先级，缺少必要符号时仍禁用 hook。

ABI 依据 Node [v26.8.1 V8 头文件](https://github.com/nodejs/node/tree/v26.8.1/deps/v8/include)和 [API 实现](https://github.com/nodejs/node/blob/v26.8.1/deps/v8/src/api/api.cc)核对：`Source` 开头仍为源码、资源名两个 `Local` 字段；无论 direct 还是 indirect handle，`Local` 存储表示均与公开 API 的 receiver 表示一致，无须读取堆对象偏移。Windows x64 V2 符号声明已通过 Clang MSVC target 核对，尚未实测 Windows/macOS 的 Node 26 注入。后续版本需保持相同导出签名和 Source 前缀布局，不保证未来版本兼容。

使用已构建的 launcher 和指定 Node 运行 UTF-8 回归检查（需要 Python 3）：

```sh
python3 scripts/test-node.py target/debug/v8_killer_launcher /path/to/node
python3 scripts/test-node.py target/release/v8_killer_launcher /path/to/node
```

检查覆盖默认 `.js`/`.cjs` 执行、ESM 导入 CJS、Unicode 文件名、中文、emoji 和内嵌 NUL，同时断言 ESM 源码未被处理。

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
