[package]
name = "foo"
version = "0.1.0"
authors = ["mark"]

[dependencies]
clap = "2.27.1" # from crates.io
rand = { git = "https://github.com/rust-lang-nursery/rand" } # from online repo
bar = { path = "../bar" } # from a path in the local filesystem

foo
├── Cargo.toml
└── src
    └── main.rs

    [package]
name = "foo"
version = "0.1.0"
authors = ["mark"]

[dependencies]
