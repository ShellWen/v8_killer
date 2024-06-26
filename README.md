<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->
<a name="readme-top"></a>
<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Don't forget to give the project a star!
*** Thanks again! Now go create something AMAZING! :D
-->



<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]
[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/O5O4RNVHA)



<!-- PROJECT LOGO -->
<br />
<div align="center">
  <h3 align="center">V8 Killer</h3>

  <p align="center">
    A <strong>powerful</strong> and highly <strong>customizable</strong> <strong>universal</strong> V8 virtual machine injector.
    <br />
    <a href="https://shellwen.github.io/v8_killer/"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/ShellWen/v8_killer/issues">Report Bug</a>
    ·
    <a href="https://github.com/ShellWen/v8_killer/issues">Request Feature</a>
  </p>
</div>

<!-- DOCUMENTATIONS -->
## Documentations

Documentations are available under [GitHub Pages](https://shellwen.github.io/v8_killer/).


<!-- ABOUT THE PROJECT -->
## About The Project

This project began with an initial idea: injecting scripts into Electron applications. There are traditionally two main approaches for accomplishing this. 
- Modifying resource files, such as .js or .asar files. However, this approach is highly invasive and cannot pass integrity checks in some software. 
- Opening a debugging port (`--inspect` or `--inspect-brk`) and injecting scripts using a debugger. However, some software may inspect this parameter or outright block it.

This project takes a different approach by hooking into the compilation functions of the V8 engine, directly modifying the source code passed to the V8 compiler. This allows scripts to be injected into the V8 engine without altering any local files or opening any debugging ports. Through testing, it has been confirmed that this method can be used with any software/framework built on the V8 engine, including but not limited to Node.js, Electron, and Deno.

Currently, this project has been tested exclusively on Linux and Windows. In theory, with minor modifications, it should be possible to run it on macOS. However, this is not currently part of our development roadmap.

This project is divided into two parts: `core` and `launcher`. The `core` constitutes the central component and represents the actual injected payload. The `launcher` is responsible for loading the payload, which is the `core`, into the target program.

On Linux, loading the payload can be accomplished simply using `LD_PRELOAD`. However, on Windows, this might require additional work, and this is where the purpose of the launcher comes into play.

So far, we support the following targets:

| Target   | Supported | Note                                                                                                       |
|----------|-----------|------------------------------------------------------------------------------------------------------------|
| Node.js  | Yes       |                                                                                                            |
| Electron | Yes       |                                                                                                            |
| CEF      | Untested  |                                                                                                            |
| Deno     | No        | Deno remove exports from V8. In future versions, we will introduce pattern matching to address this issue. |

Pattern matching is on the way. [#12](https://github.com/ShellWen/v8_killer/issues/12)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- GETTING STARTED -->
## Getting Started

This is an example of how you may give instructions on setting up your project locally. To get a local copy up and running follow these simple example steps.

<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

ShellWen - [@realShellWen](https://twitter.com/realShellWen) - me@shellwen.com

Project Link: [https://github.com/ShellWen/v8_killer](https://github.com/ShellWen/v8_killer)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/ShellWen/v8_killer.svg?style=for-the-badge
[contributors-url]: https://github.com/ShellWen/v8_killer/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/ShellWen/v8_killer.svg?style=for-the-badge
[forks-url]: https://github.com/ShellWen/v8_killer/network/members
[stars-shield]: https://img.shields.io/github/stars/ShellWen/v8_killer.svg?style=for-the-badge
[stars-url]: https://github.com/ShellWen/v8_killer/stargazers
[issues-shield]: https://img.shields.io/github/issues/ShellWen/v8_killer.svg?style=for-the-badge
[issues-url]: https://github.com/ShellWen/v8_killer/issues
[license-shield]: https://img.shields.io/github/license/ShellWen/v8_killer.svg?style=for-the-badge
[license-url]: https://github.com/ShellWen/v8_killer/blob/master/LICENSE
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/ShellWen

<!-- Anti GitCode -->
<!-- 8964 -->

[rust-badge]: https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white
[rust-url]: https://www.rust-lang.org/
[v8-badge]: https://img.shields.io/badge/V8-4B8BF5?style=for-the-badge&logo=v8&logoColor=white
[v8-url]: https://v8.dev/
