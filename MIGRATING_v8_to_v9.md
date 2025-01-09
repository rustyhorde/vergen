## If you weren't using the `git` features
1. Change the `vergen` build dependency to the latest version.

```toml
[dependencies]
#..
[build-dependencies]
# All features enabled
vergen = { version = "9.0.0", features = ["build", "cargo", "rustc", "si"] }
# or
vergen = { version = "9.0.0", features = ["build"] }
# if you wish to disable certain features
```

2. Update `build.rs` to use the version 9 updates.
```rust
use anyhow::Result;
use vergen::{
    BuildBuilder, CargoBuilder, Emitter, RustcBuilder, SysinfoBuilder,
};

pub fn main() -> Result<()> {
    Emitter::default()
        .add_instructions(&Build::all_build()?)
        .add_instructions(&Cargo::all_cargo()?)
        .add_instructions(&Rustc::all_rustc()?)
        .add_instructions(&Sysinfo::all_sysinfo()?)
        .emit()
}
```

## If you were using the `gix` feature

1. Change the `vergen` build dependency to `vergen-gix` in `Cargo.toml`. Remove `git` and `gix` from your feature list.

```toml
[dependencies]
#..
[build-dependencies]
# All features enabled
vergen-gix = { version = "1.0.0", features = ["build", "cargo", "rustc", "si"] }
# or
vergen-gix = { version = "1.0.0", features = ["build"] }
# if you wish to disable certain features
```

2. Update `build.rs` to use the version 9 updates, replacing your `vergen` use with `vergen-gix`.

```rust
use anyhow::Result;
use vergen_gix::{
    BuildBuilder, CargoBuilder, Emitter, GixBuilder, RustcBuilder, SysinfoBuilder,
};

pub fn main() -> Result<()> {
    Emitter::default()
        .add_instructions(&Build::all_build()?)
        .add_instructions(&Cargo::all_cargo()?)
        .add_instructions(&Gix::all_git()?)
        .add_instructions(&Rustc::all_rustc()?)
        .add_instructions(&Sysinfo::all_sysinfo()?)
        .emit()
}
```
## If you were using the `gitcl` feature

1. Change the `vergen` build dependency to `vergen-gitcl` in `Cargo.toml`. Remove `git` and `gitcl` from your feature list.

```toml
[dependencies]
#..
[build-dependencies]
# All features enabled
vergen-gitcl = { version = "1.0.0", features = ["build", "cargo", "rustc", "si"] }
# or
vergen-gitcl = { version = "1.0.0", features = ["build"] }
# if you wish to disable certain features
```

2. Update `build.rs` to use the version 9 updates, replacing your `vergen` use with `vergen-gitcl`.

```rust
use anyhow::Result;
use vergen_gitcl::{
    BuildBuilder, CargoBuilder, Emitter, GitclBuilder, RustcBuilder, SysinfoBuilder,
};

pub fn main() -> Result<()> {
    Emitter::default()
        .add_instructions(&Build::all_build()?)
        .add_instructions(&Cargo::all_cargo()?)
        .add_instructions(&Gitcl::all_git()?)
        .add_instructions(&Rustc::all_rustc()?)
        .add_instructions(&Sysinfo::all_sysinfo()?)
        .emit()
}
```
## If you were using the `git2` feature

1. Change the `vergen` build dependency to `vergen-git2` in `Cargo.toml`. Remove `git` and `git2` from your feature list.

```toml
[dependencies]
#..
[build-dependencies]
# All features enabled
vergen-git2 = { version = "1.0.0", features = ["build", "cargo", "rustc", "si"] }
# or
vergen-git2 = { version = "1.0.0", features = ["build"] }
# if you wish to disable certain features
```

2. Update `build.rs` to use the version 9 updates, replacing your `vergen` use with `vergen-git2`.

```rust
use anyhow::Result;
use vergen_git2::{
    BuildBuilder, CargoBuilder, Emitter, Git2Builder, RustcBuilder, SysinfoBuilder,
};

pub fn main() -> Result<()> {
    Emitter::default()
        .add_instructions(&Build::all_build()?)
        .add_instructions(&Cargo::all_cargo()?)
        .add_instructions(&Git2::all_git()?)
        .add_instructions(&Rustc::all_rustc()?)
        .add_instructions(&Sysinfo::all_sysinfo()?)
        .emit()
}
```
