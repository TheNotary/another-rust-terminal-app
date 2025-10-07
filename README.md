# Another Rust Terminal App 

Testing Making Rust and Terminal work in "Raw Mode" where keystrokes are responded to immediately and don't require the user to press the enter key.

## Creating Project

```
cargo init
# update src/main.rs
cargo add libc
# hard-code Cargo.toml to point to my fork's libc
cargo install cargo-wasix
```

## Run Natively

```
cargo run
```

## Run Wasmerly

```
cargo wasix run
```

## Setup a Release

```
cargo wasix build --release
wasmer run .
```

