# V8 Killer
**强大**且可高度**自定义**的**通用** V8 虚拟机注入器。  
  
[![Contributors][contributors-shield]][contributors-url]{target=\_blank}
[![Forks][forks-shield]][forks-url]{target=\_blank}
[![Stargazers][stars-shield]][stars-url]{target=\_blank}
[![Issues][issues-shield]][issues-url]{target=\_blank}
[![MIT License][license-shield]][license-url]{target=\_blank}
[![LinkedIn][linkedin-shield]][linkedin-url]{target=\_blank}
[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/O5O4RNVHA){target=\_blank}

## V8 Killer 是什么

该项目 ([V8 Killer][project-url]{target=\_blank}) 是由 [ShellWen][shellwen-github-url]{target=\_blank} 开发的 [V8 引擎][v8-url]{target=\_blank}
通用脚本注入方案，目前我们主要致力于解决 [Electron][electron-url]{target=\_blank} 程序的注入问题。

## Electron 注入方案比较

目前常见的 Electron 注入方案有以下几种：

|             | 开启调试端口                             | 替换资源文件                   | V8 Killer                                 |
|-------------|------------------------------------|--------------------------|-------------------------------------------|
| 原理          | 通过特殊的命令行参数或运行时发送信号开启 Devtools 调试端口 | 替换存储在硬盘上的脚本文件 / asar 资源包 | 在程序运行后注入动态链接库 inline hook，修改 V8 引擎编译脚本的逻辑 |
| 可通过完整性检查    | ✅                                  | ❌ 修改文件会导致文件摘要值改变         | ✅                                         |
| 无安全性问题      | ❌ 调试端口无法添加保护，任何程序均可注入              | ✅                        | ✅ 注入内容由配置文件指定，不对外暴露攻击面                    |
| 更新版本后无需重新适配 | ✅                                  | ❌                        | ⭕ 一般仅 Windows 平台需要                        |
| 允许修改任意脚本    | ❌                                  | ✅ 支持对原脚本替换               | ✅ 支持对原脚本替换或**修改**                         |

目前 V8 Killer 是唯一一个通用且不会破坏 Electron 程序完整性的注入方案。  

V8 Killer 的缺点主要集中在以下几点：

- Windows 平台下的 Electron 构建，默认会移除部分符号导出信息，所以需要自行逆向后填写相关函数的 EVA；
- 部分 Electron 程序可能会检查内存中加载的动态链接库列表，这会使得 V8 Killer 在目标程序中被发现。

## 开始使用
请转到 [开始使用](getting-started.md)。

## 贡献

如果你发现了代码中存在的缺陷 / Bugs，欢迎你通过 [GitHub Issues][issues-url]{target=\_blank} 提交给我们，或是直接通过
[GitHub Pull Requests][pull-requests-url]{target=\_blank} 将解决方案提交给我们。  
如果现有的代码无法满足你的需求，或是你有什么新的创意，你也可以通过 [GitHub Issues][issues-url]{target=\_blank} 告诉我们，但
需要说明的是，项目维护者的精力有限，我们可能无法总是让你满意。  
文档翻译可能存在滞后性，也可能因疏忽而发生错误，如果遇到这种情况，烦请通过 [GitHub Issues][issues-url]{target=\_blank} 提交给我们。  
如果你对 Rust 有所了解，同时希望参与该项目的开发之中，请转到 [开发](development.md)。

## 社区

如果你在使用 V8 Killer 的时候碰到问题，请前往我们的讨论页 [GitHub Discussions][discussions-url]{target=\_blank}。

**请注意：GitHub Issues 仅用于提交代码缺陷 / Bugs，请不要把使用中的问题发到 Issues，这会分散开发人员的精力。**

## 使用须知 & 免责声明

该项目仅供学习交流使用，禁止用于非法用途，否则后果自负。  
该项目不包含任何明示或暗示的用于任何目的的担保，本项目及其贡献者不对任何人使用本项目产生的任何直接或间接损失负责。  
该项目的使用者必须在遵守开源许可证的同时，仔细阅读并遵守本声明。

## 技术栈

该项目使用了一些来自社区的开源代码，我们对这些贡献者表示由衷的感谢：  

- [frida-rust](https://github.com/frida/frida-rust){target=\_blank}
- [lazy_static.rs](https://github.com/rust-lang-nursery/lazy-static.rs){target=\_blank}
- [rust-ctor](https://github.com/mmastrac/rust-ctor){target=\_blank}
- [toml-rs](https://github.com/toml-rs/toml){target=\_blank}
- [serde-rs](https://github.com/serde-rs/serde){target=\_blank}

没有他们的贡献，该项目便无法顺利完成。再次感谢他们对开源界的贡献。

## 许可协议

该项目遵循 MIT 许可协议。具体请见项目根目录下的 [LICENSE 文件][license-url]{target=\_blank}。

[shellwen-github-url]: https://github.com/ShellWen
[project-url]: https://github.com/ShellWen/v8_killer
[contributors-shield]: https://img.shields.io/github/contributors/ShellWen/v8_killer.svg?style=for-the-badge
[contributors-url]: https://github.com/ShellWen/v8_killer/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/ShellWen/v8_killer.svg?style=for-the-badge
[forks-url]: https://github.com/ShellWen/v8_killer/network/members
[stars-shield]: https://img.shields.io/github/stars/ShellWen/v8_killer.svg?style=for-the-badge
[stars-url]: https://github.com/ShellWen/v8_killer/stargazers
[issues-shield]: https://img.shields.io/github/issues/ShellWen/v8_killer.svg?style=for-the-badge
[issues-url]: https://github.com/ShellWen/v8_killer/issues
[pull-requests-url]: https://github.com/ShellWen/v8_killer/pulls
[license-shield]: https://img.shields.io/github/license/ShellWen/v8_killer.svg?style=for-the-badge
[license-url]: https://github.com/ShellWen/v8_killer/blob/master/LICENSE
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/ShellWen

[rust-badge]: https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white
[rust-url]: https://www.rust-lang.org/
[v8-badge]: https://img.shields.io/badge/V8-4B8BF5?style=for-the-badge&logo=v8&logoColor=white
[v8-url]: https://v8.dev/

[electron-url]: https://github.com/electron/electron

[discussions-url]: https://github.com/ShellWen/v8_killer/discussions
