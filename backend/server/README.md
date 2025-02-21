# Server

## Dev

For Hot-Reload, install `cargo install systemfd cargo-watch`

```sh
systemfd --no-pid -s <PORT> -- cargo watch -x 'r -- '
```

The `PORT` above as in `.env` file `ADDRESS` field

## Build Release

```sh
cargo build --release
```