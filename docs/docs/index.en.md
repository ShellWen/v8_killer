# V8 Killer

A **powerful** and highly **customizable** **universal** V8 virtual machine injector.

[![Contributors][contributors-shield]][contributors-url]{target=\_blank}
[![Forks][forks-shield]][forks-url]{target=\_blank}
[![Stargazers][stars-shield]][stars-url]{target=\_blank}
[![Issues][issues-shield]][issues-url]{target=\_blank}
[![MIT License][license-shield]][license-url]{target=\_blank}
[![LinkedIn][linkedin-shield]][linkedin-url]{target=\_blank}
[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/O5O4RNVHA){target=\_blank}

## What is V8 Killer

This project ([V8 Killer][project-url]{target=\_blank}) developed by [ShellWen][shellwen-github-url]{target=\_blank} is
a general script injection scheme for [V8 engine][v8-url]{target=\_blank}.  
Currently, we are mainly focusing on solving the injection problems of [Electron][electron-url]{target=\_blank}
programs.

## Comparison of Electron Injection Solutions

There are several common Electron injection solutions at present:

|                                    | Enable Debugging Port                                                                | Replacing Resource Files                                          | V8 Killer                                                                                                      |
|------------------------------------|--------------------------------------------------------------------------------------|-------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------|
| Principle                          | Enable Devtools debugging port by special command-line parameters or runtime signals | Replace scripts files / asar resource packages stored on disk     | Inject dynamic link libraries inline hook after program runs, modify the logic of V8 engine script compilation |
| Can pass integrity checks          | ✅                                                                                    | ❌ The modification of files will cause the digest value to change | ✅                                                                                                              |
| No security issues                 | ❌ The debugging port cannot be protected, any program can be injected                | ✅                                                                 | ✅ The contents of the injection are specified by the configuration file and do not expose the attack surface   |
| No need to re-adapt after updating | ✅                                                                                    | ❌                                                                 | ⭕ Only need on Windows platform                                                                                |
| Allows modification of any script  | ❌                                                                                    | ✅ Supports substitution of original scripts                       | ✅ Supports substitution or **modification** of original scripts                                                |

Currently, V8 Killer is the only universal injection solution that won't break the integrity of Electron programs.

The main disadvantages of V8 Killer:

- For Electron builds under Windows platform, some symbol export information is removed by default, so you need to
  reverse and fill in the EVA of relevant functions yourself;
- Some Electron programs may check the list of dynamically linked libraries loaded in memory, which may lead V8 Killer
  to be discovered in the target program.

## Getting Started

Please go to [Getting Started](getting-started.md).

## Contribution

If you find flaws / Bugs in the code, we welcome you to submit them to us via [GitHub Issues][issues-url]{target=\_blank}, or directly submit solutions to us through [GitHub Pull Requests][pull-requests-url]{target=\_blank}.  
If the existing code does not meet your needs, or if you have any new ideas, you can also tell us
via [GitHub Issues][issues-url]{target=\_blank}, but we need to point out that the time of project maintainer is
limited, we may not always be able to satisfy you.  
The translation of documents may be delayed or errors may occur due to negligence. If this happens, please report them to us via [GitHub Issues][issues-url]{target=\_blank}.  
If you know
about Rust and hope to participate in the development of this project, please go to [Development](development.md).

## Community

If you encounter problems while using V8 Killer, please go to our discussion page [GitHub Discussions][discussions-url]{target=\_blank}.

**Please note: GitHub Issues are only used to submit code defects / Bugs, please do not post usage issues to Issues,
which would distract developers.**

## User Notice & Disclaimer

This project is only for learning and communication purposes, it is forbidden to use it for illegal purposes, or you
will bear the consequences.  
This project does not contain any explicit or implied warranties for any purpose, and the
contributors of this project are not responsible for any direct or indirect losses caused by anyone using this project.  
The users of this project must read this statement carefully while complying with the open source license.

## Technology Stack

This project uses some open source code from the community, we would like to express our sincere gratitude to these
contributors:

- [frida-rust](https://github.com/frida/frida-rust){target=\_blank}
- [lazy_static.rs](https://github.com/rust-lang-nursery/lazy-static.rs){target=\_blank}
- [rust-ctor](https://github.com/mmastrac/rust-ctor){target=\_blank}
- [toml-rs](https://github.com/toml-rs/toml){target=\_blank}
- [serde-rs](https://github.com/serde-rs/serde){target=\_blank}

Without their contributions, this project would not have been successfully completed.  
Thanks again for their contributions to the open source community.

## Licensing Agreement

This project follows the MIT licensing agreement.  
For specifics, see the [LICENSE file][license-url]{target=\_blank} in the project root directory.

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
