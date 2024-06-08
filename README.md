# Rust + C

A little `C/C++` with your `Rust`. The *"Hello world"* to building Rust apps with C/C++ libs.

## Prerequisites

The easiest way to get started is by using the Visual Studio Code Remote - Containers / Codespaces [development container](.devcontainer/devcontainer.json) included in this repository. This container comes with Rust, Cargo, and several VSCode extensions pre-installed to help you get started quickly.

- For [Remote - Containers](https://aka.ms/vscode-remote/download/containers), use the **Remote-Containers: Open Repository in Container...** command which creates a Docker volume for better disk I/O on macOS and Windows.
- For Codespaces, install the [GitHub Codespaces](https://marketplace.visualstudio.com/items?itemName=GitHub.codespaces) extension in VSCode, and use the **Codespaces: Create New Codespace command**.

Once your workspace is setup, open a terminal to check everything is working:

```bash
cargo --version
```

```bash
rustc --version
```

## Build & Run

To build the project, run the following command:

```bash
cargo run
```

This will compile and link the C library using [cmake-rs](https://github.com/rust-lang/cmake-rs), generate the Rust bindings using [bindgen](https://github.com/rust-lang/rust-bindgen), and run the application.

To pass arguments to the program, use the following command:

```bash
cargo run -- <arg>
```
