# Installing Rust
This document notes the basic steps to install rust.

## gcc
> Why is gcc required? :(

```sh
$ sudo apt-get install rust
```

## Rustup Toolchain
Rustup is the installer and version management tool for Rust. It manages the
various build versions of rust.

To install:
```sh
curl https://sh.rustup.rs -sSf | sh
```

To update Rust:
```sh
rustup update
```

## Cargo
Cargo is a Rust build tool and packet manager. It is installed by defualt along
with Rustup.

## Confuguring `PATH`
All Rust tools (`rustc`, `rustup`, `cargo` etc.), are installed to
`~/.cargo/bin/`.

Add the following to `~/.bashrc`:
```sh
export PATH="$HOME/.cargo/bin:$PATH"
```
## References
* [Getting Started](https://www.rust-lang.org/learn/get-started)
* [Installation](https://www.rust-lang.org/tools/install)
