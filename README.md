# [Bevy](https://bevyengine.org/) Ball Shooter

# [DEMO](https://volodalexey.github.io/bevy-wasm-ball-shooter/)

Originally created by [frans](https://github.com/pyrbin) as [ball_shooter](https://github.com/pyrbin/ball_shooter)

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
animation for move down
animation for snap position
menu button in game
button to switch snap or bounce
show next ball to shoot
draw full line to first contact