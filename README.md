# Sloughi

![Crates.io](https://img.shields.io/crates/v/sloughi)

<img align="right" width="180" alt="Sloughi Dog" src="https://user-images.githubusercontent.com/983020/160320457-6dbdd901-01c4-4cd0-8d3c-c758cb4f615b.jpeg" />

A tiny crate to make it easy to share and apply Git hooks for Rust projects. Inspired by [Husky](https://github.com/typicode/husky).
- Zero dependencies.
- No magic, uses Git's `config core.hooksPath` to set a custom hooks path. Use native/regular shell scripts for git hooks.
- Makes use of [Cargo's build script](https://doc.rust-lang.org/cargo/reference/build-scripts.html).
- IDE/Code editor agnostic.
- Customizable setup.

[Sloughi](https://en.wikipedia.org/wiki/Sloughi) is an ancient breed of domesticated dog originating from North Africa (i.e. Algeria, where I live 👋).

# Install 
- Add sloughi to your `build-dependencies` (not `dev-dependencies`):

  ```toml
  [build-dependencies]
  sloughi = "0.2.0"
  ```
- Create a `build.rs` file at the root of your project to install Sloughi (besides `Cargo.toml`, not inside `src/`):

  ```rust
  use sloughi::Sloughi;

  fn main() {
      let _ = Sloughi::new().install(); // This will fail silently. Won't interrupt the build.
  }
  ```
That's it!

The next time `cargo build` is triggered (by VSCode, or by running `cargo run`/`cargo test`), you will notice a `.sloughi` folder is created with a sample `pre-commit` hook inside. 

## Customizing the install
This crate uses the builder pattern, you can chain options on top `::new()` to adjust the install:
```rust
let _ = Sloughi::new()
    .custom_path(".git_hooks").   // Choose a custom Git hooks relative path (default is ".sloughi")
    .ignore_env("CI").            // Ignore setup when `CI` environment variable is set (like in CircleCI ..etc)
    .ignore_env("GITHUB_ACTIONS") // Do not run in Github Actions as well
    .install();
```

### Fail the build
The above snippets for `build.rs` will not interrupt the build in case Sloughi install failed (e.g. not a git repo, permission error ..etc). It's explicitly silenced with `let _ =`, you can handle the error however you like, or just yell with:

```rust
Sloughi::new().install().expect("Sloughi install failed");
```

# F.A.Q

### Why use `build.rs`? 
Cargo lacks install hooks (like `postinstall` in npm's `package.json`), this makes it challenging to have a way to share git hooks automatically in a upstream repository.

Cargo's build script is a nice & transparent way to:
- Act on the project at build/compile time.
- Customize the setup without extra config files and formats.

### What happens when cargo build is run multiple times?

The `install()` call is [idempotent](https://en.wikipedia.org/wiki/Idempotence). It won't modify the existing setup and hooks.

### Will this slow down my builds? 
No:
- `build.rs` itself is only built when it's changed (by default).
- The setup will first check if you already have `.sloughi` (or the custom path) created. If so, it skips installs.

### Does it run in release mode? 
No, release mode is a no-op.


# TODO:
- [ ] Optional feature flags. I.e. conventional commits as pre-commit, rustfmt as pre-commit. 
- [ ] A [optiona] companion binary to manage hooks. 
- [ ] Check Cargo workspaces compatiblity. 
- [ ] Introduce the `uninstall()` call to the exported `Sloughi` struct.
- [ ] Integration tests.
