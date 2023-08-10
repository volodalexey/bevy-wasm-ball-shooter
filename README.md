# [Bevy](https://bevyengine.org/) Ball Shooter

# [DEMO](https://volodalexey.github.io/bevy-wasm-ball-shooter/)

Inspired by [ball_shooter](https://github.com/pyrbin/ball_shooter)

Refactored to use WASM with touch/mouse support

## Compile to WASM (WebAssembly) and build for browser

### Add WebAssembly support to your Rust installation
```sh
rustup target install wasm32-unknown-unknown
```

### Install [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) CLI
```sh
cargo install wasm-bindgen-cli
```

### Run build script

```sh
./wasm/build.sh
```

TODO
draw full line to first contact
improve collision: second collision is snap?
improve clusters: use only physics connections?
responsive layout
improve performance: show only visible rows + physics from visible row