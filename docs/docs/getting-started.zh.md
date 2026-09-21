本文假设你已经对相关的基础知识有所了解。  

## 结构
V8 Killer 分为 `core` 核心部分，与 `launcher` 启动器部分。  
前者为运行在目标进程中的部分，会在模块加载的时候 inline hook V8 引擎的编译相关函数，后者负责启动程序并将 `core` 加载进目标程序。  

## 获取二进制文件

[//]: # (目前，我们提供预构建版本，发布在 [GitHub Releases][github-releases-url]{target=\_blank} 中)

手动运行 [GitHub Actions][github-actions-build-url]{target=\_blank} CI 可生成 Linux x64 与 Windows x64 GNU release 构建产物，保留 7 天。PR 仅上传回归 JSON 报告。新版 CI 配置尚未远程运行。
Windows GNU 还需下载配套 `runtime-x86_64-pc-windows-gnu` 产物，将其中 DLL 放到目标 Node 可执行文件旁或其 DLL 搜索路径中。
如需自行构建，请参考 [开发](development.md)。

CI 已配置官方 Node 22/24/26 的 Linux 与 Windows GNU/Wine 覆盖，具体本地已测版本见开发文档。原生 Windows/MSVC 与真实 Electron 注入尚未验证。macOS 为实验性支持，没有 CI 测试。

## 编写配置文件

配置文件是一个 [TOML][toml-url]{target=\_blank} 文件，其中包括了函数定位器与注入规则。  
我们目前暂时还未提供 TOML 文件的 JSON Schema，请参考 [/examples/configs][config-examples-url]{target=\_blank} 中给出的示例文件编写。

## 使用启动器启动

启动器会根据环境变量查找配置文件，你需要设置 `V8_KILLER_CONFIG_FILE_PATH` 环境变量，并将它的值指向配置文件的**绝对路径**。  

### Linux / macOS
macOS 使用实验性实现；下方示例仅在 Linux 上验证。
这是一段示例脚本，已在 Arch Linux 下测试通过：  
```bash
#!/usr/bin/env bash
export V8_KILLER_CONFIG_FILE_PATH=/path/to/config/file/config.toml
v8_killer_launcher "/usr/bin/node" "/path/to/js/main.js"
```

### Windows
你可以通过 `PowerShell` 脚本，或 `cmd` 批处理启动 `launcher`：  
```powershell
# PowerShell
$env:V8_KILLER_CONFIG_FILE_PATH = "C:\path\to\config\file\config.toml"
& "C:\path\to\v8_killer_launcher.exe" "C:\path\to\executable\node.exe" -- "C:\path\to\js\main.js"
```
```batch
:: cmd
set V8_KILLER_CONFIG_FILE_PATH=C:\path\to\config\file\config.toml
"C:\path\to\v8_killer_launcher.exe" "C:\path\to\executable\node.exe" -- "C:\path\to\js\main.js"
```
请注意，在 Windows 中，允许配置全局环境变量，但我们不需要这么做，我们只需要在脚本中设置临时环境变量即可。

[toml-url]: https://toml.io/
[github-releases-url]: https://github.com/ShellWen/v8_killer/releases
[github-actions-build-url]: https://github.com/ShellWen/v8_killer/actions/workflows/build.yaml
[config-examples-url]: https://github.com/ShellWen/v8_killer/tree/master/examples/configs
