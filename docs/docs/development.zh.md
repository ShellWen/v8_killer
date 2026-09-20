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
