# Another Rust Terminal App 

Testing Making Rust and Terminal in Raw Mode...


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
cargo wasix build --release
wasmer run .
```

