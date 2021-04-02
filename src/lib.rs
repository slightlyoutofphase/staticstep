#![no_std]
#![feature(step_trait, step_trait_ext)]

use core::iter::Step;
use core::ops::{
  Bound::{Excluded, Included, Unbounded},
  RangeBounds,
};

/// The [`Iterator`](core::iter::Iterator)-implementing struct through which the functionality of
/// [`inc_by`](crate::IntoIncBy::inc_by) actually operates. Not useful by itself.
pub struct IncBy<T: Copy + Default + Step, const STEP: usize> {
  start: T,
  end: T,
}

/// The [`Iterator`](core::iter::Iterator)-implementing struct through which the functionality of
/// [`dec_by`](crate::IntoDecBy::dec_by) actually operates. Not useful by itself.
pub struct DecBy<T: Copy + Default + Step, const STEP: usize> {
  start: T,
  end: T,
}

impl<T: Copy + Default + Step, const STEP: usize> IncBy<T, STEP> {
  #[inline(always)]
  fn new<R: RangeBounds<T>>(bounds: R) -> IncBy<T, STEP> {
    let start = match bounds.start_bound() {
      Included(&idx) => idx,
      Excluded(&idx) => idx,
      Unbounded => Default::default(),
    };
    let end = match bounds.end_bound() {
      Included(&idx) => Step::forward(idx, STEP),
      Excluded(&idx) => Step::forward(idx, STEP - 1),
      Unbounded => Default::default(),
    };
    IncBy { start, end }
  }
}

impl<T: Copy + Default + Step, const STEP: usize> DecBy<T, STEP> {
  #[inline(always)]
  fn new<R: RangeBounds<T>>(bounds: R) -> DecBy<T, STEP> {
    let start = match bounds.end_bound() {
      Included(&idx) => idx,
      Excluded(&idx) => Step::forward(idx, 1),
      Unbounded => Step::forward(Default::default(), 1),
    };
    let end = match bounds.start_bound() {
      Included(&idx) => Step::forward(idx, STEP),
      Excluded(&idx) => Step::forward(idx, STEP + 1),
      Unbounded => Default::default(),
    };
    DecBy { start, end }
  }
}

impl<T: Copy + Default + Step, const STEP: usize> Iterator for IncBy<T, STEP> {
  type Item = T;

  #[inline(always)]
  fn next(&mut self) -> Option<T> {
    if self.start <= Step::backward(self.end, STEP) {
      let res = Some(self.start);
      self.start = Step::forward(self.start, STEP);
      res
    } else {
      None
    }
  }
}

impl<T: Copy + Default + Step, const STEP: usize> Iterator for DecBy<T, STEP> {
  type Item = T;

  #[inline(always)]
  fn next(&mut self) -> Option<T> {
    if Step::forward(self.start, STEP) <= self.end {
      self.end = Step::backward(self.end, STEP);
      Some(self.end)
    } else {
      None
    }
  }
}

/// A subtrait of [`RangeBounds<T>`](core::ops::RangeBounds) where `T` is `Copy + Default + Step`
/// that turns implementers of it into an instance of [`IncBy`][crate::IncBy] when
/// [`inc_by`](crate::IntoIncBy::inc_by) is called.
pub trait IntoIncBy<T: Copy + Default + Step>: RangeBounds<T> {
  /// Functionally equivalent to what [`step_by`](core::iter::Iterator::step_by) does when it is
  /// called through a primitive range, but written specifically with primitive ranges in mind such
  /// that it optimizes identically to a `while` loop in every case the author of this crate has
  /// examined so far.
  ///
  /// # Example usage:
  /// ```
  /// # use staticstep::*;
  /// // Exclusive, so prints: 'A C E'
  /// for i in ('A'..'G').inc_by::<2>() {
  ///   print!("{} ", i);
  /// }
  ///
  /// // Inclusive, so prints: '0 4 8 12'
  /// for i in (0isize..=12isize).inc_by::<4>() {
  ///   print!("{} ", i);
  /// }
  fn inc_by<const STEP: usize>(self) -> IncBy<T, STEP>;
}

/// A subtrait of [`RangeBounds<T>`](core::ops::RangeBounds) where `T` is `Copy + Default + Step`
/// that turns implementers of it into an instance of [`DecBy`][crate::DecBy] when
/// [`dec_by`](crate::IntoDecBy::dec_by) is called.
pub trait IntoDecBy<T: Copy + Default + Step>: RangeBounds<T> {
  /// Functionally equivalent to what [`step_by`](core::iter::Iterator::step_by) does when it is
  /// called through a primitive range, but specifically in reverse and operating directly on
  /// ranges that are themselves syntactically "backwards", while offering the same level of
  /// optimizability as [`inc_by`](crate::IntoIncBy::inc_by).
  ///
  /// # Example usage:
  /// ```
  /// # use staticstep::*;
  /// // Exclusive, so prints: 'G E C'
  /// for i in ('G'..'A').dec_by::<2>() {
  ///   print!("{} ", i);
  /// }
  ///
  /// // Inclusive, so prints: '4 1 -2'
  /// for i in (4isize..=-4isize).dec_by::<3>() {
  ///   print!("{} ", i);
  /// }
  fn dec_by<const STEP: usize>(self) -> DecBy<T, STEP>;
}

/// The actual implementation of [`IntoIncBy`](crate::IntoIncBy) for
/// [`RangeBounds`](core::ops::RangeBounds).
impl<T: Copy + Default + Step, R: RangeBounds<T>> IntoIncBy<T> for R {
  #[inline(always)]
  fn inc_by<const STEP: usize>(self) -> IncBy<T, STEP> {
    IncBy::<T, STEP>::new(self)
  }
}

/// The actual implementation of [`IntoDecBy`](crate::IntoDecBy) for
/// [`RangeBounds`](core::ops::RangeBounds).
impl<T: Copy + Default + Step, R: RangeBounds<T>> IntoDecBy<T> for R {
  #[inline(always)]
  fn dec_by<const STEP: usize>(self) -> DecBy<T, STEP> {
    DecBy::<T, STEP>::new(self)
  }
}
