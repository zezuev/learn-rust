## Build System

Cargo is Rust's build system and package manager.

To create a new project using Cargo, type

    cargo new PROJECT_NAME

It expects your source files to live inside the _src_ directory.

To build a cargo project, type

    cargo build

This creates an executable file in _target/debug/PROJECT_NAME_

To run the build, type

    cargo run

To build your project for release, use

    cargo build --release

This produces an executable in _target/release_ instead.
