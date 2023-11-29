This document assumes that you are already familiar with the related basic knowledge.

## Structure
V8 Killer consists of a `core` part and a `launcher` part.  
The former part runs inside the target process and inline hooks the V8 engine's compilation-related functions when the module is loaded. The latter is responsible for starting the program and loading the `core` into the target program.

## Obtain Binary Files

[//]: # (At present, we provide pre-built versions, released on [GitHub Releases][github-releases-url]{target=\_blank})

At present, we do not provide pre-built versions, but you can find continuously built binary files in [GitHub Actions][github-actions-build-url]{target=\_blank}.  
If you need to build it yourself, please refer to [Development](development.md).

## Writing Configuration Files

The configuration file is a [TOML][toml-url]{target=\_blank} file, which includes function locators and injection rules.  
At present, we have not yet provided a JSON Schema for TOML files. Please refer to the example files given in [/examples/configs][config-examples-url]{target=\_blank}.

## Using the Launcher

The launcher will look for the configuration file based on environment variables. You need to set the `V8_KILLER_CONFIG_FILE_PATH` environment variable and point its value to the **absolute path** of the configuration file.  

### Linux / macOS
Here is an example script that has been tested on Arch Linux:
```bash
#!/usr/bin/env bash
export V8_KILLER_CONFIG_FILE_PATH=/path/to/config/file/config.toml
v8_killer_launcher "/usr/bin/node" "/path/to/js/main.js"
```

### Windows
You can use `PowerShell` scripts or `cmd` batch files to start the launcher:  
```powershell
# PowerShell
$env:V8_KILLER_CONFIG_FILE_PATH = "C:\path\to\config\file\config.toml"
Start-Process -FilePath "C:\path\to\executable\node.exe" -ArgumentList "C:\path\to\js\main.js" -NoNewWindow
```
```batch
:: cmd
set V8_KILLER_CONFIG_FILE_PATH=C:\path\to\config\file\config.toml
start "C:\path\to\executable\node.exe" "C:\path\to\js\main.js"
```
Please note that in Windows, it is allowed to configure global environment variables, but we don't need to do so. We only need to set temporary environment variables in the script.

[toml-url]: https://toml.io/
[github-releases-url]: https://github.com/ShellWen/v8_killer/releases
[github-actions-build-url]: https://github.com/ShellWen/v8_killer/actions/workflows/build.yaml
[config-examples-url]: https://github.com/ShellWen/v8_killer/tree/master/examples/configs