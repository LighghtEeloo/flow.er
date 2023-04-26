# Flow.er

## After 2 years of silence...

I've grown and learnt a lot from my experience with programming and software development, and most importantly, how to make a notebook app meaningful. The ecosystem has dramtically evolved, and I'm glad to see that. I'm planning to rewrite the project in my free time.

Below is the corpus of what was done 2 years ago, when I was still young.

---

A *notebook* app integrated with *todo-list* utility.

Project *flow.er* is a Rust WASM app running in browser. Taking advantage of Yew and Trunk, it provides good development experience.

***Notice: this repo is under heavy development, and is currently not usable. To see the demo version, you can try `flower-yew-lock` branch, which is the last version of this software and is already usable, and follow the instructions below.***

## Screenshots

![](./docs/Screenshot_1.png)

![](./docs/Screenshot_2.png)

## Try It!

### Dev Install

This projects uses `trunk` as a Rust WASM application bundler.

First, install [`rust & cargo`](https://www.rust-lang.org/learn/get-started). Note that `cargo` is auto-installed if you follow the instructions.

Next, install [`trunk`](https://github.com/thedodd/trunk) by the following (or just follow the instructions provided on the link)
```bash
$ cargo install --locked trunk
$ cargo install wasm-bindgen-cli
```

### Dev Serve

After following intructions above, you'll be able to serve the app locally.

First, clone / download this repo: https://github.com/LighghtEeloo/flow.er.git.

Then just switch to `flow.er/flow_vase/flow_yew` folder and run `trunk serve`. This will serve the app on `127.0.0.1:9720`, which can be visited via your preferred browser.

The whole process will be like:
```bash
$ git clone https://github.com/LighghtEeloo/flow.er.git && cd flow.er/flow_vase/flow_yew
$ trunk serve
```
And then visit `127.0.0.1:9720` via your browser.

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


## Supported Browsers

Chrome, Firefox and Edge are (roughly) tested and all seem to be working well. 

However, only desktop versions are considered for now.

## Disclaimer

This is a completely personal, non-profit project, mainly aiming at learning rust-lang, yew and the surrounding toolchains. I can try my best, but I won't be responsible for any of your potential data loss. 

For now, this software isn't data-safe. Use at your own risk. This may get better as I develop, and you're welcomed to help improving it.


## Vision

The final goal of this project is to create a *notebook* and *mind-map* app with is integrated with *todo-list* and *agenda* views. 

It will be supporting patches to incrementally save your previous work, denoted by *time-capsule* function.

