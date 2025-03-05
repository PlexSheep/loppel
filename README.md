# loppel

![Project badge](https://img.shields.io/badge/language-Rust-blue.svg)
![Crates.io License](https://img.shields.io/crates/l/loppel)
![GitHub Release](https://img.shields.io/github/v/release/PlexSheep/loppel)
![GitHub language count](https://img.shields.io/github/languages/count/PlexSheep/loppel)
[![Rust CI](https://github.com/PlexSheep/hedu/actions/workflows/cargo.yaml/badge.svg)](https://github.com/PlexSheep/hedu/actions/workflows/cargo.yaml)


Simple local file backups with a bit of compression

* [GitHub](https://github.com/PlexSheep/loppel)
* [crates.io](https://crates.io/crates/loppel)
* [docs.rs](https://docs.rs/crate/loppel/)

loppel creates backup copies of files and directories right where they are
– just add `.bak`, `.bak.d`, or `.tar.zstd` if you need them smaller. No cloud,
no complicated configs, just quick local copies when you need them.

Perfect for that "let me backup this config before I break it" moment or
"I should save a copy before I try this" situation. Restores are just as
straightforward, and optionally clean up after themselves.

Think of it as cp with a bit more smarts about what you're copying and how
you might want it compressed.

