#![feature(test)]

extern crate test;

use test::{black_box, Bencher};

use staticstep::*;

#[bench]
fn inc_by_exclusive(b: &mut Bencher) {
  let mut j = 0usize;
  b.iter(|| {
    for i in (0usize..32768usize).inc_by::<16usize>() {
      j += black_box(i);
    }
  });
}

#[bench]
fn inc_by_inclusive(b: &mut Bencher) {
  let mut j = 0usize;
  b.iter(|| {
    for i in (0usize..=32768usize).inc_by::<16usize>() {
      j += black_box(i);
    }
  });
}

#[bench]
fn dec_by_exclusive(b: &mut Bencher) {
  let mut j = 0usize;
  b.iter(|| {
    for i in (32768usize..0usize).dec_by::<16usize>() {
      j += black_box(i);
    }
  });
}

#[bench]
fn dec_by_inclusive(b: &mut Bencher) {
  let mut j = 0usize;
  b.iter(|| {
    for i in (32768usize..=0usize).dec_by::<16usize>() {
      j += black_box(i);
    }
  });
}

#[bench]
fn step_by_inc_exclusive(b: &mut Bencher) {
  let mut j = 0usize;
  b.iter(|| {
    for i in (0usize..32768usize).step_by(16usize) {
      j += black_box(i);
    }
  });
}

#[bench]
fn step_by_inc_inclusive(b: &mut Bencher) {
  let mut j = 0usize;
  b.iter(|| {
    for i in (0usize..=32768usize).step_by(16usize) {
      j += black_box(i);
    }
  });
}

#[bench]
fn step_by_dec_exclusive(b: &mut Bencher) {
  let mut j = 0usize;
  b.iter(|| {
    for i in (16usize..32784usize).step_by(16usize).rev() {
      j += black_box(i);
    }
  });
}

#[bench]
fn step_by_dec_inclusive(b: &mut Bencher) {
  let mut j = 0usize;
  b.iter(|| {
    for i in (0usize..=32768usize).rev().step_by(16usize) {
      j += black_box(i);
    }
  });
}
