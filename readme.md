# Flow.er

A *notebook* and *mind-map* app, integrated with *todo-list* and *calendar* views. And it has *time-capsule* function.

Flow.er is a Rust WASM app running in browser. It uses local storage tech to save cache, and supports downloading all progress / exporting to clipboard. One can import data back with source code mode.

## Dev Install

This projects uses `trunk` as a Rust WASM application bundler.

First, install [`rust & cargo`](https://www.rust-lang.org/learn/get-started). Note that `cargo` is auto-installed if you follow the instructions.

Next, install [`trunk`](https://github.com/thedodd/trunk) by the following (or just follow the instructions provided on the link)
```bash
$ cargo install --locked trunk
$ cargo install wasm-bindgen-cli
```

Maybe you will fail to compile on your first run with some error messages like:
```bash
[INFO]: Checking for the Wasm target...
Error: wasm32-unknown-unknown target not found!
```

Don't panic. Simply add
```bash
rustup target add wasm32-unknown-unknown
```
and everything will be fine.

Feel free to fire an issue if anything troubles you (❁´◡`❁)

## Vision Board

In the future the following features may be implemented:
- Cloud / server saved.
