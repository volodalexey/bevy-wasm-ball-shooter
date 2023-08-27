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

- Magneitic force can be applied only to dynamic rigid bodies
- Kinematic rigid bodies used as attraction point for rigid bodies
- Balls with nearest distance have more attraction factor than other

TODO
move down shold not depend on success cluster, because success can be achieved some time after
split to count score system
shoot multiple projectiles
responsive layout and scale
show first row partially
show next projectile from one side and multiple items
UI bottom show info
switch next projectile
sprites for balls and buttons
WASM audio