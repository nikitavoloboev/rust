# shell-capture

> Run command passed as is but capture stdout and stderr

Currently making above case work with this fish command:

```
env CARGO_TERM_COLOR=always cargo watch -q -- sh -c "tput reset && cargo run -q" 2>&1 | tee ~/log/cmd.log
```

The `env` is there to pass the colors and capture it too.

[src/main.rs](src/main.rs) tries to make this into one binary for better DX.

## Run

```
cargo watch -q -- sh -c "tput reset && cargo run -q ls"
```

Where `ls` is command you are testing on.

## Build

```
cargo build --release
```
