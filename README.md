# rust-connect4

# Getting Started

## Dependencies

```bash
cargo install trunk wasm-bindgen-cli
```

```bash
rustup target add wasm32-unknown-unknown
```

[Install wasm-pack](https://rustwasm.github.io/wasm-pack/)

## Running the Server

```
mongod --port 27017
```

```bash
cargo run
```

## Running the Client

```bash
trunk serve --open
```

## Debugging

```
mongod --port 27017
```

```bash
cargo run -- cli
```
