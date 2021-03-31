[![Latest Version]][crates.io] ![Rustc Version nightly]

[Latest Version]: https://img.shields.io/crates/v/staticstep.svg
[crates.io]: https://crates.io/crates/staticstep
[Rustc Version nightly]: https://img.shields.io/badge/rustc-nightly-lightgray.svg
[![Build status](https://ci.appveyor.com/api/projects/status/dw7nt480aewaux76/branch/master?svg=true)](https://ci.appveyor.com/project/slightlyoutofphase/staticstep/branch/master)

Provides `inc_by()` and `dec_by()` (which are similar to `step_by()` but actually designed to optimize well for numeric ranges) for all instances of `RangeBounds<T: Copy + Default + Step>`.

This readme will likely be expanded upon in the future, but for now just run `cargo bench` on the repository to get an idea of why you might be inclined to use this crate.

**Minimum supported Rust version:** this is a nightly-only crate at the moment due to the use of
the `Step` trait, which has not yet been stabilized.

**`#![no_std]` compatibility:** this crate is fully `#![no_std]` compatible by default.

A basic usage example:

```rust
use staticstep::*;

// Apart from aiming to provide a properly-optimized Rust equivalent to the sort of C-style for-loop
// that ends in `i += number` or `i -= number` as opposed to `i++` or `i--`, this crate also aims to
// (and does) support backwards ranges in a meaningful way that's logically equivalent to how
// forwards ranges are generally dealt with in Rust.

fn main() {
  // Exclusive, so 48 is the last number printed.
  for i in (0..64).inc_by::<16>() {
    print!("{} ", i);
  }
  println!("");
  // Inclusive, so 64 is the last number printed.
  for i in (0..=64).inc_by::<16>() {
    print!("{} ", i);
  }
  println!("");
  // Exclusive, so 16 is the last number printed.
  for i in (64..0).dec_by::<16>() {
    print!("{} ", i);
  }
  println!("");
  // Inclusive, so 0 is the last number printed.
  for i in (64..=0).dec_by::<16>() {
    print!("{} ", i);
  }
  // Note that `inc_by` will always immediately return `None` if given a reverse range, while
  // `dec_by` will always immediately return `None` if given a "normal" forwards range.
}
```

**License:**

Licensed under either the <a href="LICENSE-MIT">MIT license</a> or version 2.0 of the <a href="LICENSE-APACHE">Apache License</a>. Your choice as to which!
Any source code contributions will be dual-licensed in the same fashion.
