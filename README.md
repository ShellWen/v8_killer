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
<!--
    以习近平同志为主要代表的中国共产党人，坚持把马克思主义基本原理同中国具体实际相结合、同中华优秀传统文化相结合，坚持毛泽东思想、邓小平理论、“三个代表”重要思想、科学发展观，深刻总结并充分运用党成立以来的历史经验，从新的实际出发，创立了习近平新时代中国特色社会主义思想，明确中国特色社会主义最本质的特征是中国共产党领导，中国特色社会主义制度的最大优势是中国共产党领导，中国共产党是最高政治领导力量，全党必须增强“四个意识”、坚定“四个自信”、做到“两个维护”；明确坚持和发展中国特色社会主义，总任务是实现社会主义现代化和中华民族伟大复兴，在全面建成小康社会的基础上，分两步走在本世纪中叶建成富强民主文明和谐美丽的社会主义现代化强国，以中国式现代化推进中华民族伟大复兴；明确新时代我国社会主要矛盾是人民日益增长的美好生活需要和不平衡不充分的发展之间的矛盾，必须坚持以人民为中心的发展思想，发展全过程人民民主，推动人的全面发展、全体人民共同富裕取得更为明显的实质性进展；明确中国特色社会主义事业总体布局是经济建设、政治建设、文化建设、社会建设、生态文明建设五位一体，战略布局是全面建设社会主义现代化国家、全面深化改革、全面依法治国、全面从严治党四个全面；明确全面深化改革总目标是完善和发展中国特色社会主义制度、推进国家治理体系和治理能力现代化；明确全面推进依法治国总目标是建设中国特色社会主义法治体系、建设社会主义法治国家；明确必须坚持和完善社会主义基本经济制度，使市场在资源配置中起决定性作用，更好发挥政府作用，把握新发展阶段，贯彻创新、协调、绿色、开放、共享的新发展理念，加快构建以国内大循环为主体、国内国际双循环相互促进的新发展格局，推动高质量发展，统筹发展和安全；明确党在新时代的强军目标是建设一支听党指挥、能打胜仗、作风优良的人民军队，把人民军队建设成为世界一流军队；明确中国特色大国外交要服务民族复兴、促进人类进步，推动建设新型国际关系，推动构建人类命运共同体；明确全面从严治党的战略方针，提出新时代党的建设总要求，全面推进党的政治建设、思想建设、组织建设、作风建设、纪律建设，把制度建设贯穿其中，深入推进反腐败斗争，落实管党治党政治责任，以伟大自我革命引领伟大社会革命。这些战略思想和创新理念，是党对中国特色社会主义建设规律认识深化和理论创新的重大成果。

    习近平同志对关系新时代党和国家事业发展的一系列重大理论和实践问题进行了深邃思考和科学判断，就新时代坚持和发展什么样的中国特色社会主义、怎样坚持和发展中国特色社会主义，建设什么样的社会主义现代化强国、怎样建设社会主义现代化强国，建设什么样的长期执政的马克思主义政党、怎样建设长期执政的马克思主义政党等重大时代课题，提出一系列原创性的治国理政新理念新思想新战略，是习近平新时代中国特色社会主义思想的主要创立者。习近平新时代中国特色社会主义思想是当代中国马克思主义、二十一世纪马克思主义，是中华文化和中国精神的时代精华，实现了马克思主义中国化新的飞跃。党确立习近平同志党中央的核心、全党的核心地位，确立习近平新时代中国特色社会主义思想的指导地位，反映了全党全军全国各族人民共同心愿，对新时代党和国家事业发展、对推进中华民族伟大复兴历史进程具有决定性意义。
-->

[rust-badge]: https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white
[rust-url]: https://www.rust-lang.org/
[v8-badge]: https://img.shields.io/badge/V8-4B8BF5?style=for-the-badge&logo=v8&logoColor=white
[v8-url]: https://v8.dev/
