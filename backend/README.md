# d42x

You need Bun.
For Hot-Reload, install `cargo install systemfd cargo-watch`

## Usage

### Dev

Entry `d42x-server`

```sh
$ systemfd --no-pid -s 9876 -- cargo watch -x 'r -- '
```

Entry `d42x-server/wwwroot`

```sh
$ bun dev
```