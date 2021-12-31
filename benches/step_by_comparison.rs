#![feature(test)]

extern crate test;

use test::{black_box, Bencher};

use staticstep::*;

// We use explicit `usize` variables everywhere here just to
// make sure the compiler doesn't do any integer-type
// optimizations that would make the benchmark less fair.

#[bench]
fn inc_by_exclusive(b: &mut Bencher) {
  b.iter(|| {
    let mut j = 0usize;
    for i in (0usize..32768usize).inc_by::<16usize>() {
      j += black_box(i);
    }
    let _ = black_box(j);
  });
}

#[bench]
fn inc_by_inclusive(b: &mut Bencher) {
  b.iter(|| {
    let mut j = 0usize;
    for i in (0usize..=32768usize).inc_by::<16usize>() {
      j += black_box(i);
    }
    let _ = black_box(j);
  });
}

#[bench]
fn dec_by_exclusive(b: &mut Bencher) {
  b.iter(|| {
    let mut j = 0usize;
    for i in (32768usize..0usize).dec_by::<16usize>() {
      j += black_box(i);
    }
    let _ = black_box(j);
  });
}

#[bench]
fn dec_by_inclusive(b: &mut Bencher) {
  b.iter(|| {
    let mut j = 0usize;
    for i in (32768usize..=0usize).dec_by::<16usize>() {
      j += black_box(i);
    }
    let _ = black_box(j);
  });
}

#[bench]
fn while_loop_inc_exclusive(b: &mut Bencher) {
  b.iter(|| {
    let mut i = 0usize;
    let mut j = 0usize;
    while i < 32768usize {
      j += black_box(i);
      i += 16usize;
    }
    let _ = black_box(j);
  });
}

#[bench]
fn while_loop_inc_inclusive(b: &mut Bencher) {
  b.iter(|| {
    let mut i = 0usize;
    let mut j = 0usize;
    while i <= 32768usize {
      j += black_box(i);
      i += 16usize;
    }
    let _ = black_box(j);
  });
}

#[bench]
fn while_loop_dec_exclusive(b: &mut Bencher) {
  b.iter(|| {
    let mut j = 0usize;
    let mut i = 32768isize;
    while i > 0isize {
      j += black_box(i as usize);
      i -= 16isize;
    }
    let _ = black_box(j);
  });
}

#[bench]
fn while_loop_dec_inclusive(b: &mut Bencher) {
  b.iter(|| {
    let mut j = 0usize;
    let mut i = 32768isize;
    while i >= 0isize {
      j += black_box(i as usize);
      i -= 16isize;
    }
    let _ = black_box(j);
  });
}

#[bench]
fn step_by_inc_exclusive(b: &mut Bencher) {
  b.iter(|| {
    let mut j = 0usize;
    for i in (0usize..32768usize).step_by(16usize) {
      j += black_box(i);
    }
    let _ = black_box(j);
  });
}

#[bench]
fn step_by_inc_inclusive(b: &mut Bencher) {
  b.iter(|| {
    let mut j = 0usize;
    for i in (0usize..=32768usize).step_by(16usize) {
      j += black_box(i);
    }
    let _ = black_box(j);
  });
}

#[bench]
fn step_by_dec_exclusive(b: &mut Bencher) {
  b.iter(|| {
    // There's no other way to write this such that it goes
    // over the exact same numbers as the other versions and
    // also stops in the same place.
    let mut j = 0usize;
    for i in (16usize..32784usize).step_by(16usize).rev() {
      j += black_box(i);
    }
    let _ = black_box(j);
  });
}

#[bench]
fn step_by_dec_inclusive(b: &mut Bencher) {
  b.iter(|| {
    let mut j = 0usize;
    for i in (0usize..=32768usize).rev().step_by(16usize) {
      j += black_box(i);
    }
    let _ = black_box(j);
  });
}
