# Discord Forge

A Discord bot for converting smash ultimate and smash for wii u filetypes.

![](https://cdn.discordapp.com/attachments/447940880922574848/570386121385574400/unknown.png)

## Build Requirements

* Rust nightly
```
rustup install nightly
```
* Recent `openssl` version
* `cmake` (see below)
* `libsamplerate` (see below)

Ubuntu dependency setup:

```
sudo apt install -y cmake libsamplerate-dev 
```

## Building

Needs openssl 1.10+, may require a `cargo clean` before rebuilding if getting ssl errors even after installing.

```
cargo +nightly build
```

use the %update command within discord to install the needed non-static dependencies. See %help for more information.
