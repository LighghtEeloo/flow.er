# Flow.er

A *notebook* and *mind-map* app, integrated with *todo-list* and *calendar* views. And it has *time-capsule* function.

Flow.er is a Rust WASM app running in browser. It uses local storage tech to save cache, and supports downloading all progress / exporting to clipboard. One can import data back with source code mode.

***Notice: this repo is under heavy development, and is currently not usable. To see the demo version, you can try `tracer-yew-lock` branch, which is the last version of this software and is already usable. For detailed concerns, please read [this](https://github.com/LighghtEeloo/flow.er/issues/1#issuecomment-785710178) and follow the instructions below.***

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

## Dev Serve

After following intructions above, you'll be able to serve the app locally.

First, clone / download this repo: https://github.com/LighghtEeloo/flow.er.git.

Then just switch to the repo's root folder and run `trunk serve`. This will serve the app on `127.0.0.1:9720`, which you can visit via your preferred browser.

The whole process will be like:
```bash
$ git clone https://github.com/LighghtEeloo/flow.er.git && cd flow.er
$ trunk serve
```
And visit `127.0.0.1:9720` via your browser.

## Supported Browsers

Chrome, Firefox and Edge are (roughly) tested and all seem to be working well. 

However, only desktop versions are considered for now.

## Disclaimer

This is a completely personal, non-profit project. I can try my best, but I won't be responsible for any of your data loss. 

For now, this software isn't data-safe. Use at your own risk. This may get better as I develop, and you're welcomed to help improving it.

## Vision Board

In the future the following features may be implemented:
- Cloud / server saved.
