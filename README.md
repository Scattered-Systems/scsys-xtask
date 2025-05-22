# scsys-xtask

[![crate.io (license)](https://img.shields.io/crates/l/scsys-xtask?logo=rust&style=for-the-badge)

[![crates.io](https://img.shields.io/crates/v/scsys-xtask?logo=rust&style=for-the-badge)](https://crates.io/crates/scsys-xtask)
[![docs.rs](https://img.shields.io/docsrs/scsys-xtask?style=for-the-badge&logo=rust)](https://docs.rs/scsys-xtask)

[![Docker Image Version](https://img.shields.io/docker/v/jo3mccain/scsys-xtask?sort=semver&arch=amd64&style=for-the-badge&logo=docker)](https://hub.docker.com/r/jo3mccain/scsys-xtask)

***

_**Warning: The application is currently in the early stages of development and is not yet ready for production use.**_

`scsys-xtask` is a custom build system for the [`scsys-io`](https://scsys.io) ecosystem, designed to support a variety of workflows that empower our all-in-one platform, [`eryon`](https://docs.rs/eryon).

## Features

- Docker support
- Native support
  - Linux support (x86_64-unknown-linux-gnu)
  - MacOS support (x86_64-apple-darwin)
  - Windows support (x86_64-pc-windows-msvc)
  - FreeBSD support (x86_64-unknown-freebsd)
  - OpenBSD support (x86_64-unknown-openbsd)
  - DragonFly BSD support (x86_64-unknown-dragonfly)
  - NetBSD support (x86_64-unknown-netbsd)
- NixOs support
- WebAssembly support
  - Emscripten support (wasm32-unknown-unknown)
  - WebAssembly System Interface (WASI) support (wasm32-wasip1, wasm32-wasip2)

## Usage

## Library: `scsys-xtask`

Add the following to your `Cargo.toml`:

```toml
[dependencies.scsys-xtask]
version = "0.0.*"
features = ["full"]
```

## Binary: `scsysx`

### Installation

```bash
cargo binstall scsys-xtask
```

### Running the server

```bash
pzzld serve --port 8080
```

## Getting Started

### Setting up

Ensure that `rustup` and all installed toolchains are updated:

```bash
rustup update
```

Optionally, instal the `wasm32-*` targets for WebAssembly development:

```bash
rustup target add wasm32-unknown-unknown wasm32-wasip1 wasm32-wasip2
```

### Building from the source

Get started by cloning the repository:

```bash
git clone https://github.com/FL03/scsys-xtask.git --branch main
```

Then, navigate to the project directory:

```bash
cd scsys-xtask
```

#### Native

For native development, you can run the server using cargo:

```bash
cargo run --locked --release --features full --bin pzzld --
```

#### WebAssembly

##### WebAssembly System Interface (wasi)

Build the project using the wasm32 target:

```bash
cargo build --locked --workspace --release --features wasi --target wasm32-wasip2
```

### Docker

You can also build the project using Docker. Start by building the Docker image:

```bash
docker buildx build --platform linux/amd64 -t jo3mccain/scsys-xtask:latest -f ./Dockerfile .
```

Then, run the Docker container:

```bash
docker run -d -it --rm -p 8080:8080 -v $(pwd):/app jo3mccain/scsys-xtask:latest
```

This will start the server and bind it to port 8080 on your host machine. You can access the server by navigating to `http://localhost:8080` in your web browser.

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.
