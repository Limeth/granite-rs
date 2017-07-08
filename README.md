# granite-rs
An effort to create granite bindings for Rust

# Notes for contributors

## Generating C bindings to Granite (`src/ffi.rs`)

You need `bindgen` installed to generate the bindings. You may install it using this command:
```bash
cargo install bindgen
```

First, you need to install the dependencies using the following command:
```bash
sudo apt install libgranite-dev
```

Then you should be able to generate the bindings:
```bash
pkg-config --cflags glib-2.0 gtk+-3.0 gee-0.8 | xargs bindgen /usr/include/granite/granite.h -o src/ffi.rs --whitelist-type '_?[Gg]ranite.*' --whitelist-function 'granite.*' --
```
