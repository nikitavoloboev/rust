# Rust

> Rust crates and other code

## Published crates

- [log_macro](https://github.com/nikitavoloboev/log_macro) - Macro to print variable(s) with values nicely (stripped from release builds)

## Setup

[Bun](https://bun.sh/) is used to run things.

Assumes [rust](https://www.rust-lang.org/tools/install) with [cargo](https://doc.rust-lang.org/cargo/) is installed.

## Run

```
bun dev
```

Runs: `cargo watch -q -- sh -c "tput reset && cargo run -q"`

I prefer `cargo watch -q -- sh -c "tput reset && cargo run -q"` over just `cargo run` as it will rerun rust code on rust file changes and keep output always on top of the terminal.

## Test

```
bun run test
```

Runs: `cargo watch -q -- sh -c "tput reset && cargo test -q"`

## Publish crates

```
cargo publish
```

## Contribute

Always open to useful ideas or fixes in form of issues or PRs.

Can [open new issue](../../issues/new/choose) (search [existing issues](../../issues) first) or [start discussion](../../discussions).

It's okay to submit draft PR as you can get help along the way to make it merge ready.

Join [Discord](https://discord.com/invite/TVafwaD23d) for more indepth discussions on this repo and [others](https://github.com/nikitavoloboev#src).

### 🖤

[Support on GitHub](https://github.com/sponsors/nikitavoloboev) or look into [other projects](https://nikiv.dev/projects).

[![Discord](https://img.shields.io/badge/Discord-100000?style=flat&logo=discord&logoColor=white&labelColor=black&color=black)](https://discord.com/invite/TVafwaD23d) [![X](https://img.shields.io/badge/nikitavoloboev-100000?logo=X&color=black)](https://x.com/nikitavoloboev) [![nikiv.dev](https://img.shields.io/badge/nikiv.dev-black)](https://nikiv.dev)
