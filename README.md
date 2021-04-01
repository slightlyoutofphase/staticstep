[![Latest Version]][crates.io] ![Rustc Version nightly]

[Latest Version]: https://img.shields.io/crates/v/staticstep.svg
[crates.io]: https://crates.io/crates/staticstep
[Rustc Version nightly]: https://img.shields.io/badge/rustc-nightly-lightgray.svg
[![Build status](https://ci.appveyor.com/api/projects/status/dw7nt480aewaux76/branch/master?svg=true)](https://ci.appveyor.com/project/slightlyoutofphase/staticstep/branch/master)

Provides truly zero-cost alternatives to `Iterator::step_by` for both incrementing and decrementing any type that satisfies `RangeBounds<T: Copy + Default + Step>`.

The assembly code generated for the two trait methods this crate implements, `inc_by` and `dec_by`, should in the overwhelming majority of cases be nearly or completely identical
to the assembly code that would be generated for an incrementing "step"-based `while` loop or decrementing "step"-based `while` loop, respectively. If you come across a scenario where
it is not, please feel free to open an issue about it.

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
