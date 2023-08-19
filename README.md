# [Bevy](https://bevyengine.org/) Ball Shooter

# [DEMO](https://volodalexey.github.io/bevy-wasm-ball-shooter/)

Main goal is WASM with touch/mouse support

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
walk through physics connections for cluster
shoot multiple projectiles
responsive layout and scale
show first row partially
show next projectile from one side and multiple items
UI bottom show info
switch next projectile
sprites for balls and buttons
WASM audio