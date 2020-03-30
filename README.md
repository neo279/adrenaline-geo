# adrenaline-geo

Simple HTTP(S) client embedded for [l2j-geoserver-rest](https://github.com/neo279/l2j-geoserver-rest).

## Usage

Get `adrenaline_geo.dll` from [releases](https://github.com/neo279/l2j-geoserver-rest/releases) into Adrenaline folder, next to `Adrenaline.exe` file.

Load Adrenaline, and start script `GeoEngineTest.txt`

Script Console should show:

```
0
1
```

## Build DLL

Get Rust 32-bit via [rustup](https://www.rust-lang.org/learn/get-started) and install 32bit dev kit (default host triple) `i686-pc-windows-msvc`.

```
cargo build

# or if you have x64 toolchain as default
rustup run stable-i686-pc-windows-msvc cargo build
```

Library needs to be build for 32bit target!

This repository is mostly for educational purpurose.
