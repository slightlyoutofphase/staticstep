#![feature(test)]

extern crate test;

use test::{black_box, Bencher};

use staticstep::*;

#[bench]
fn inc_by_exclusive(b: &mut Bencher) {
  let mut j = 0;
  b.iter(|| {
    for i in (0..32768).inc_by::<16>() {
      j += black_box(i);
    }
  });
}

#[bench]
fn inc_by_inclusive(b: &mut Bencher) {
  let mut j = 0;
  b.iter(|| {
    for i in (0..=32768).inc_by::<16>() {
      j += black_box(i);
    }
  });
}

#[bench]
fn dec_by_exclusive(b: &mut Bencher) {
  let mut j = 0;
  b.iter(|| {
    for i in (32768..0).dec_by::<16>() {
      j += black_box(i);
    }
  });
}

#[bench]
fn dec_by_inclusive(b: &mut Bencher) {
  let mut j = 0;
  b.iter(|| {
    for i in (32768..=0).dec_by::<16>() {
      j += black_box(i);
    }
  });
}

#[bench]
fn step_by_inc_exclusive(b: &mut Bencher) {
  let mut j = 0;
  b.iter(|| {
    for i in (0..32768).step_by(16) {
      j += black_box(i);
    }
  });
}

#[bench]
fn step_by_inc_inclusive(b: &mut Bencher) {
  let mut j = 0;
  b.iter(|| {
    for i in (0..=32768).step_by(16) {
      j += black_box(i);
    }
  });
}

#[bench]
fn step_by_dec_exclusive(b: &mut Bencher) {
  let mut j = 0;
  b.iter(|| {
    for i in (16..32784).step_by(16).rev() {
      j += black_box(i);
    }
  });
}

#[bench]
fn step_by_dec_inclusive(b: &mut Bencher) {
  let mut j = 0;
  b.iter(|| {
    for i in (0..=32768).rev().step_by(16) {
      j += black_box(i);
    }
  });
}
