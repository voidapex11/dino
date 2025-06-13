# Dino

[![dependency status](https://deps.rs/repo/github/emilk/eframe_template/status.svg)](https://deps.rs/repo/github/emilk/eframe_template)
[![Build Status](https://github.com/emilk/eframe_template/workflows/CI/badge.svg)](https://github.com/emilk/eframe_template/actions?workflow=CI)

This is a copy of the google dino game made using [eframe](https://github.com/emilk/egui/tree/master/crates/eframe), a framework for writing apps using [egui](https://github.com/emilk/egui/).

The advantage of using eframe was that I could write the app once, and build it for web, windows, or linux without the need for large rewrites.

## development

How to work on this code:

### Testing locally

Make sure you are using the latest version of stable rust by running `rustup update`.

`cargo run --release`

On Linux you need to first run:

`sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev`

On Fedora Rawhide you need to run:

`dnf install clang clang-devel clang-tools-extra libxkbcommon-devel pkg-config openssl-devel libxcb-devel gtk3-devel atk fontconfig-devel`

### Web Locally

You can compile your app to [WASM](https://en.wikipedia.org/wiki/WebAssembly) and publish it as a web page.

We use [Trunk](https://trunkrs.dev/) to build for web target.

1. Install the required target with `rustup target add wasm32-unknown-unknown`.
2. Install Trunk with `cargo install --locked trunk`.
3. Run `trunk serve` to build and serve on `http://127.0.0.1:8080`. Trunk will rebuild automatically if you edit the project.
4. Open `http://127.0.0.1:8080/index.html#dev` in a browser. See the warning below.
