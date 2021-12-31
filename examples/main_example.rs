#![feature(bench_black_box)]

use core::hint::black_box;
use staticstep::*;

// You can run this with "--emit asm" as an easy way of comparing the assembly output from this
// crate's functions to the assembly output from a traditional loop.

#[inline(never)]
fn inc_by_exclusive() {
  let mut j = 0;
  for i in (0..32768).inc_by::<16>() {
    j += black_box(i);
  }
  println!("{}", j);
}

#[inline(never)]
fn inc_by_inclusive() {
  let mut j = 0;
  for i in (0..=32768).inc_by::<16>() {
    j += black_box(i);
  }
  println!("{}", j);
}

#[inline(never)]
fn dec_by_exclusive() {
  let mut j = 0;
  for i in (32768..0).dec_by::<16>() {
    j += black_box(i);
  }
  println!("{}", j);
}

#[inline(never)]
fn dec_by_inclusive() {
  let mut j = 0;
  for i in (32768..=0).dec_by::<16>() {
    j += black_box(i);
  }
  println!("{}", j);
}

#[inline(never)]
fn while_loop_inc_exclusive() {
  let mut j = 0;
  let mut i = 0;
  while i < 32768 {
    j += black_box(i);
    i += 16;
  }
  println!("{}", j);
}

#[inline(never)]
fn while_loop_inc_inclusive() {
  let mut j = 0;
  let mut i = 0;
  while i <= 32768 {
    j += black_box(i);
    i += 16;
  }
  println!("{}", j);
}

#[inline(never)]
fn while_loop_dec_exclusive() {
  let mut j = 0usize;
  let mut i = 32768isize;
  while i > 0 {
    j += black_box(i as usize);
    i -= 16;
  }
  println!("{}", j);
}

#[inline(never)]
fn while_loop_dec_inclusive() {
  let mut j = 0usize;
  let mut i = 32768isize;
  while i >= 0 {
    j += black_box(i as usize);
    i -= 16;
  }
  println!("{}", j);
}

fn main() {
  inc_by_exclusive();
  inc_by_inclusive();
  dec_by_exclusive();
  dec_by_inclusive();
  while_loop_inc_exclusive();
  while_loop_inc_inclusive();
  while_loop_dec_exclusive();
  while_loop_dec_inclusive();
}
