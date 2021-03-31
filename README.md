[![Latest Version]][crates.io] ![Rustc Version nightly]

[Latest Version]: https://img.shields.io/crates/v/staticstep.svg
[crates.io]: https://crates.io/crates/staticstep
[Rustc Version nightly]: https://img.shields.io/badge/rustc-nightly-lightgray.svg

Provides `inc_by()` and `dec_by()` (which are similar to `step_by()` but actually designed to optimize well for numeric ranges) for all instances of `RangeBounds<T: Copy + Default + Step>`.

This readme will likely be expanded upon in the future, but for now just run `cargo bench` on this repository to get an idea of why you might be inclined to use this crate.

**Minimum supported Rust version:** this is a nightly-only crate at the moment due to the use of
the `Step` trait, which has not yet been stabilized.

A basic usage example:

```rust
use staticstep::*;

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