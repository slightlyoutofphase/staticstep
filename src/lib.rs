#![no_std]
#![feature(step_trait, step_trait_ext)]

use core::iter::Step;
use core::ops::{
  Bound::{Excluded, Included, Unbounded},
  RangeBounds,
};

pub struct IncBy<T: Copy + Default + Step, const STEP: usize> {
  start: T,
  end: T,
}

pub struct DecBy<T: Copy + Default + Step, const STEP: usize> {
  start: T,
  end: T,
}

impl<T: Copy + Default + Step, const STEP: usize> IncBy<T, STEP> {
  #[inline(always)]
  fn new<R: RangeBounds<T>>(bounds: R) -> IncBy<T, STEP> {
    let start = match bounds.start_bound() {
      Included(&idx) => idx,
      Excluded(&idx) => Step::forward(idx, STEP),
      Unbounded => Default::default(),
    };
    let end = match bounds.end_bound() {
      Included(&idx) => Step::forward(idx, STEP),
      Excluded(&idx) => idx,
      Unbounded => Default::default(),
    };
    IncBy { start, end }
  }
}

impl<T: Copy + Default + Step, const STEP: usize> DecBy<T, STEP> {
  #[inline(always)]
  fn new<R: RangeBounds<T>>(bounds: R) -> DecBy<T, STEP> {
    let start = match bounds.start_bound() {
      Included(&idx) => idx,
      Excluded(&idx) => Step::backward(idx, STEP),
      Unbounded => Default::default(),
    };
    let end = match bounds.end_bound() {
      Included(&idx) => idx,
      Excluded(&idx) => Step::forward(idx, STEP),
      Unbounded => Default::default(),
    };
    DecBy { start, end }
  }
}

impl<T: Copy + Default + Step, const STEP: usize> Iterator for IncBy<T, STEP> {
  type Item = T;

  #[inline(always)]
  fn next(&mut self) -> Option<T> {
    if let Some(remaining) = Step::backward_checked(self.end, STEP) {
      if remaining >= self.start {
        let res = Some(self.start);
        self.start = Step::forward(self.start, STEP);
        res
      } else {
        None
      }
    } else {
      None
    }
  }
}

impl<T: Copy + Default + Step, const STEP: usize> Iterator for DecBy<T, STEP> {
  type Item = T;

  #[inline(always)]
  fn next(&mut self) -> Option<T> {
    if self.start == self.end {
      let res = Some(self.start);
      // This will make us return `None` for the following `next()` call.
      self.end = Step::forward(self.end, STEP);
      res
    } else if let Some(remaining) = Step::backward_checked(self.start, STEP) {
      if remaining >= self.end {
        let res = Some(self.start);
        self.start = remaining;
        res
      } else {
        None
      }
    } else {
      None
    }
  }
}

pub trait IntoIncBy<T: Copy + Default + Step>: RangeBounds<T> {
  fn inc_by<const STEP: usize>(self) -> IncBy<T, STEP>;
}

pub trait IntoDecBy<T: Copy + Default + Step>: RangeBounds<T> {
  fn dec_by<const STEP: usize>(self) -> DecBy<T, STEP>;
}

impl<T: Copy + Default + Step, R: RangeBounds<T>> IntoIncBy<T> for R {
  #[inline(always)]
  fn inc_by<const STEP: usize>(self) -> IncBy<T, STEP> {
    IncBy::<T, STEP>::new(self)
  }
}

impl<T: Copy + Default + Step, R: RangeBounds<T>> IntoDecBy<T> for R {
  #[inline(always)]
  fn dec_by<const STEP: usize>(self) -> DecBy<T, STEP> {
    DecBy::<T, STEP>::new(self)
  }
}
