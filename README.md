# appy
Declarative cross-platform UI framework in Rust for native applications. More info in [this article](https://medium.com/@limikael/declarative-ui-programming-in-rust-for-native-applications-d93862886545).

![a93a9729-828e-4270-a13e-95fec7d94188](https://user-images.githubusercontent.com/902911/228746390-06f3fbf4-4cf2-40cb-bef9-d1b30ddd1e23.jpeg)

### Running the examples locally

Should be as easy as cloning the code, cd:ing into one of the example dirs (e.g. the `hello` example) and running:

    cargo run

The `hello` example is, by default, using [SDL](https://www.libsdl.org/) to render its graphics. As an alternative, you can edit the `Cargo.toml` file
and change the line:

    appy = {path="../..",features=["sdl"]}

...to...

    appy = {path="../..",features=["glutin"]}

And Appy will use [Glutin](https://crates.io/crates/glutin) instead. The graphics pipeline in the Rust ecosystem is experiencing
a kind of cambrian explosion at the moment, which is why it is good to be flexible possible when it comes to working with different
underlying libraries and toolchains.

Another very interesting library is [Miniquad](https://crates.io/crates/miniquad) which might be supported in the future.

### Running the examples on Android

If you got it to build and run the "Hello World" app, the next step would be to get it to run on Android. Depending on if you use Glutin or SDL,
try the following:

* **With SDL**
  * Install [cargo-sdl-apk](https://github.com/limikael/cargo-sdl-apk).
  * Run `cargo sdl-apk run` from inside the crate.

* **With Glutin**
  * Install [cargo-apk](https://crates.io/crates/cargo-apk).
  * Run `cargo apk run` from inside the crate.
