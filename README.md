# appy
Declarative UI framework in Rust

Read more at:

### Running the examples locally

Should be as easy as cloning the code, cd:ing into one of the example dirs and running:

    cargo run

### Running the examples on android

Prerequisites:

* The SDL source, clone it from [here](https://github.com/libsdl-org/SDL). Make sure you have the `release-2.26.x` branch.
* Java. Muse be jdk17 (doesn't work with jdk19).
* Android SDK with command line tools.
* Android NDK.

In the future I'm planning to simplify this so the tool can download these dependencies automatically, but it doesn't do that at the moment.

1. Install appy as a bin. Run in the cloned root folder:
   ```
   cargo install --path=.
   ```
2. Set the environment variables:
   * `ANDROID_HOME` pointing to the Android SDK.
   * `ANDROID_NDK_HOME` pointing to the Android NDK.
   * `SDL` pointing to the SDL source dir.
3. Run from inside an example dir:
   ```
   appy build-android
   ```
   This should give you and APK file in:
   ```
   target/android-project/app/build/outputs/apk/debug/app-debug.apk
   ```
4. Run the following command and adb will be used to upload the app to your phone and start it:
   ```
   appy run-android
   ```
   
