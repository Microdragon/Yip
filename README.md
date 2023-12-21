# Yet *another* IPC Language

[![License](https://img.shields.io/github/license/Microdragon/Yip?style=flat-square)](LICENSE)

YIP defines how Microdragon services communicate with each other and client processes.

YIP is an Interface Definition Language (IDL), a Application Binary Interface (ABI) and a Tool tieing it all together.
The Yip Tool can read in `.yip` IDL files and generate bindings for a specified programming language.
The actual bytes send between services and client processes are defined by the YIP ABI but the generated code makes it as simple as calling a method.

**NOTE:** Yip is still very much in development and currently only the parser is implemented. Contributions are very much welcome and developer as well as internal documentation is to follow.

<!--
## Install

All install options are also listed on our [website](https://yip.microdragon.rs/).

Cargo Binstall:
```console
$ cargo binstall yip
```

## Usage

To generate bindings for an interface, run `yip generate` with your desired language plugin and input IDL.

Rust Example:
```console
$ yip generate -p https://yap.microdragon.rs/microdragon/rust-backend/0.0.0/plugin.wasm https://yap.microdragon.rs/microdragon/example/0.0.0
```

The YIP Tool also has a Language Server Protocol (LSP) Server integrated. To start it on stdio, simply run:

```console
$ yip lsp
```

## Contributing

If you are interested in contributing to YIP, please read our [CONTRIBUTING.md](CONTRIBUTING.md) to get started.
-->
## License

&copy; 2023 Rain \<rain@microdragon.rs\>.

This project is licensed under either of

- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([`LICENSE-APACHE`](LICENSE-APACHE))
- [MIT license](https://opensource.org/licenses/MIT) ([`LICENSE-MIT`](LICENSE-MIT))

at your option.

The [SPDX](https://spdx.dev) license identifier for this project is `MIT OR Apache-2.0`.
