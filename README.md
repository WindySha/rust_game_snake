# Introduction
This is an implementation of the traditional game snake, using the game engine bevy.  
  
This is a cross platform game，it can run on MacOs, Linux, Windows and Web.

Play it online: [snake](https://windysha.github.io/rust_game_snake/)

# Build
## Linux/Mac/Win
```
$ cargo build
or
$ cargo run
```
## Web
First, install toolchains:  
```
$ rustup target add wasm32-unknown-unknown
$ cargo install wasm-bindgen-cli
```

Then, compile wasm and copy file to out directory:  
```
$ ./wasm-build.cmd
```

Run the website:  
```
$ cd out/
$ python3 -m http.server
```
# Reference
1. [tetris](https://github.com/NightsWatchGames/tetris)
2. [use act to debug action](https://kaimingwan.com/2023/05/23/swmf9mte55lyb5h9/)
3. [rust-wasm-github](https://github.com/plippe/rust-wasm-github) / [blog: rust-wasm-github](https://plippe.github.io/blog/2021/07/12/rust-wasm-github.html)
