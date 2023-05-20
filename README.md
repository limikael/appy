# appy
Declarative cross-platform UI framework in Rust for native applications. More info in [this article](https://medium.com/@limikael/declarative-ui-programming-in-rust-for-native-applications-d93862886545).

![a93a9729-828e-4270-a13e-95fec7d94188](https://user-images.githubusercontent.com/902911/228746390-06f3fbf4-4cf2-40cb-bef9-d1b30ddd1e23.jpeg)

### Running the examples locally

Should be as easy as cloning the code and running one of these:

    cargo run --example hello --release
    cargo run --example calculator --release
    ...

Check the examples folder for more examples. Btw, without the `--release` flag, it will still work, but
your impression of the performance will probably not be that good.

### Running the examples on Android

First install and set up [cargo-sdl-apk](https://crates.io/crates/cargo-sdl-apk).

Then, running the examples on android should be as easy as locally, almost:

    cargo sdl-apk run --example hello

### Changing rendering backend

Appy is, by default, using [SDL](https://www.libsdl.org/) to render its graphics. It also supports 
[Glutin](https://crates.io/crates/glutin), which can be turned on as a feature. Check `platform-examples/test-glutin`
for a project that is set up to use it. Running locally is done in the same way, i.e. with `cargo run`, but if you want to use Glutin on Android you need to use [cargo-apk](https://crates.io/crates/cargo-apk) instead of cargo-sdl-apk.

### Live reload

If you want live reload during development this can be done with [cargo-watch](https://crates.io/crates/cargo-watch). Install it and run:
```
cargo watch -x run
```
