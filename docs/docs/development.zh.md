# 开发

## 构建 Rust 项目

通过 [rustup](https://rustup.rs/) 安装稳定版 Rust，并安装当前平台的原生编译器和链接器。Windows 使用 MSVC 工具链及 Visual Studio C++ 构建工具。

在仓库根目录构建工作区：

```sh
cargo build --release
```

这会将 `crates/core` 共享库和 `crates/launcher` 可执行文件构建到 `target/release/`。核心库使用自动下载的 Frida 开发工具包，因此首次构建需要联网。

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
