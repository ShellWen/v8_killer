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



<!-- PROJECT LOGO -->
<br />
<div align="center">
  <h3 align="center">V8 Killer</h3>

  <p align="center">
    A tool that can inject any js into the V8 VM
    <br />
    <a href="https://github.com/ShellWen/v8_killer/wiki"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/ShellWen/v8_killer/issues">Report Bug</a>
    ·
    <a href="https://github.com/ShellWen/v8_killer/issues">Request Feature</a>
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>



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
| Bun      | No        | Bun remove exports from V8. In future versions, we will introduce pattern matching to address this issue.  |

<p align="right">(<a href="#readme-top">back to top</a>)</p>



### Built With

* [![Rust][rust-badge]][rust-url]
* [![V8][v8-badge]][v8-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- GETTING STARTED -->
## Getting Started

This is an example of how you may give instructions on setting up your project locally. To get a local copy up and running follow these simple example steps.

### Prerequisites

This is an example of how to list things you need to use the software and how to install them.
* rust
  > Please follow Rust's official installation instructions: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

### Building

1. Clone the repo
   ```sh
   git clone https://github.com/ShellWen/v8_killer.git
   ```
2. Run
    ```sh
    cargo build
    ```
    to get a debug build, or
    ```sh
    cargo build --release
    ```
    to get a release build.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- USAGE EXAMPLES -->
## Usage

In short, you need pass environment variable `V8_KILLER_CONFIG_FILE_PATH` to the launcher, and the launcher will load the config file and inject the payload into the target program.  
Here we use Node.js as an example.
```sh
V8_KILLER_CONFIG_FILE_PATH=path_to_config.toml v8_killer_launcher /use/bin/node path_to_target.js
```
Example config files can be found in the `examples/configs/` directory.    
Currently, v8 killer only supports toml format config files.

_For more examples, please refer to the [Wiki](https://github.com/ShellWen/v8_killer/wiki)_

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ROADMAP -->
## Roadmap

WIP

See the [open issues](https://github.com/ShellWen/v8_killer/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

ShellWen - [@realShellWen](https://twitter.com/realShellWen) - me@shellwen.com

Project Link: [https://github.com/ShellWen/v8_killer](https://github.com/ShellWen/v8_killer)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

* [ShellWen](https://github.com/ShellWen)

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

[rust-badge]: https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white
[rust-url]: https://www.rust-lang.org/
[v8-badge]: https://img.shields.io/badge/V8-4B8BF5?style=for-the-badge&logo=v8&logoColor=white
[v8-url]: https://v8.dev/
