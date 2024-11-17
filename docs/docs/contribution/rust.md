# Run clippy before submitting a PR for reviewing

Clippy is a nice tool to check common pitfalls and anti-patterns.
Its lints get often turned into lints of the compiler.

```bash
cargo clippy --no-deps --workspace --exclude ferrous-sdk 
```

# Formatting with `rustfmt`

`rustfmt` is used to format the complete rust codebase and only on weird
formatting cases, the `#[rustfmt::skip]` annotation should be used.

Run `rustfmt` with the following parameters:

```bash
rustfmt --config imports_granularity="Module,group_imports=StdExternalCrate" $FILE
```

# Use and mod prevalence

Module declarations should always be declared after the use statements.

**Do**:
```rust
use std::fs::create_dir;
****
use uuid::Uuid;

mod bar;
mod foo;
```

**Don't**:
```rust
mod bar;
mod foo;

use std::fs::create_dir;

use uuid::Uuid;
```

# Module-local imports / reexports

Use statements should either use the `crate`, `super` or `self` as top-level
path-segments instead of implicit relative paths.

This is important as `rustfmt` would move the corresponding use statements
in the "external crate" section.

**Do**:
```rust
use self::bar::*;

mod bar;
```

**Don't**:
```rust
use bar::*;

mod bar;
```