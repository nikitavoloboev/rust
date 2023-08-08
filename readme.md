# Rust

> Rust crates + testing code

## File structure

- [bin](bin) - binaries in rust
- [src](src) - testing out rust code
  - [main.rs](src/main.rs) - main entry point
- [try](try) - testing out rust projects

## Setup

Everything is driven using [bun](https://bun.sh/) commands.

Assumes [rust](https://www.rust-lang.org/tools/install) is installed.

## Run

```
bun run dev
```

Runs: `cargo watch -q -- sh -c "tput reset && cargo run -q"`

I prefer `cargo watch -q -- sh -c "tput reset && cargo run -q"` over just `cargo run` as it will rerun rust code on rust file changes and keep output always on top of the terminal.

## Test

```
bun run test
```

Runs: `cargo watch -q -- sh -c "tput reset && cargo test -q"`

## Publish crates

> TODO:

## Contribute

The tasks to do are outlined in [existing issues](../../issues) and in [tasks below](#tasks) (sorted by priority).

If issue/idea you have is not there, [open new issue](../../issues/new/choose) or [start discussion](../../discussions).

Any PR with code/doc improvements is welcome. ✨

Join [Discord](https://discord.com/invite/TVafwaD23d) for more indepth discussions on this repo and [others](https://github.com/nikitavoloboev#src).

## Tasks

- move markdown-parsing/sqlite/tinybase code from [LA electron code](https://github.com/learn-anything/electron-version)
  - [rusqlite](https://github.com/rusqlite/rusqlite) for sqlite
  - [markdown-rs](https://github.com/wooorm/markdown-rs) for markdown parsing
  - `bun run test` will run test suite, make tests pass
- copy [this test](https://github.com/learn-anything/electron-version/blob/main/test/wiki.test.ts) in rust and make it pass
- add nix

### ♥️

[Support on GitHub](https://github.com/sponsors/nikitavoloboev) or look into [other projects](https://nikiv.dev/projects).

[![MIT](http://bit.ly/mitbadge)](https://choosealicense.com/licenses/mit/) [![Twitter](http://bit.ly/nikitatweet)](https://twitter.com/nikitavoloboev)
