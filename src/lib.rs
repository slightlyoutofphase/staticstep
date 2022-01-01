#![no_std]
#![feature(core_intrinsics, step_trait)]

use core::intrinsics::{likely, unlikely};
use core::iter::Step;
use core::ops::{
  Bound::{Excluded, Included, Unbounded},
  RangeBounds,
};

use crate::utils::max_value;

mod utils;

/// The [`Iterator`](core::iter::Iterator)-implementing struct through which the functionality of
/// [`inc_by`](crate::IntoIncBy::inc_by) actually operates. Not useful by itself.
pub struct IncBy<T: Copy + Default + Step, const STEP: usize> {
  start: T,
  end: T,
  had_overflow: bool,
  was_unbound: bool,
}

/// The [`Iterator`](core::iter::Iterator)-implementing struct through which the functionality of
/// [`dec_by`](crate::IntoDecBy::dec_by) actually operates. Not useful by itself.
pub struct DecBy<T: Copy + Default + Step, const STEP: usize> {
  start: T,
  end: T,
  had_overflow: bool,
}

impl<T: Copy + Default + Step, const STEP: usize> IncBy<T, STEP> {
  #[inline(always)]
  fn new<R: RangeBounds<T>>(bounds: R) -> IncBy<T, STEP> {
    let start = match bounds.start_bound() {
      Included(&idx) => idx,
      Excluded(&idx) => idx,
      Unbounded => Default::default(),
    };
    let mut had_overflow = false;
    let mut was_unbound = false;
    let end = match bounds.end_bound() {
      Included(&idx) => {
        if let Some(res) = Step::forward_checked(idx, STEP) {
          res
        } else {
          had_overflow = true;
          idx
        }
      }
      Excluded(&idx) => {
        if let Some(res) = Step::forward_checked(idx, STEP - 1) {
          res
        } else {
          had_overflow = true;
          idx
        }
      }
      Unbounded => {
        was_unbound = true;
        Step::forward(Default::default(), max_value(&start))
      }
    };
    IncBy {
      start,
      end,
      had_overflow,
      was_unbound,
    }
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
    let mut had_overflow = false;
    let end = match bounds.start_bound() {
      Included(&idx) => {
        if let Some(res) = Step::forward_checked(idx, STEP) {
          res
        } else {
          had_overflow = true;
          idx
        }
      }
      Excluded(&idx) => {
        if let Some(res) = Step::forward_checked(idx, STEP + 1) {
          res
        } else {
          had_overflow = true;
          idx
        }
      }
      Unbounded => Default::default(),
    };
    DecBy {
      start,
      end,
      had_overflow,
    }
  }
}

impl<T: Copy + Default + Step, const STEP: usize> Iterator for IncBy<T, STEP> {
  type Item = T;

  #[inline(always)]
  fn next(&mut self) -> Option<T> {
    if unlikely(self.had_overflow) {
      self.had_overflow = false;
      Some(self.start)
    } else if let Some(end_back) = Step::backward_checked(self.end, STEP) {
      if likely(self.start <= end_back) {
        let res = Some(self.start);
        self.start = Step::forward(self.start, STEP);
        res
      } else if unlikely(self.was_unbound) {
        self.was_unbound = false;
        Some(self.start)
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
    if unlikely(self.had_overflow) {
      self.had_overflow = false;
      Some(self.end)
    } else if let Some(start_forward) = Step::forward_checked(self.start, STEP) {
      if likely(start_forward <= self.end) {
        self.end = Step::backward(self.end, STEP);
        Some(self.end)
      } else {
        None
      }
    } else {
      None
    }
  }
}

/// A subtrait of [`RangeBounds<T>`](core::ops::RangeBounds) where `T` is `Copy + Default + Step`
/// that turns implementers of it into an instance of [`IncBy`][crate::IncBy] when
/// [`inc_by`](crate::IntoIncBy::inc_by) is called. Currently, the blanket implementation of this
/// trait exported from the crate for `RangeBounds` is the only possible useful implementation, but
/// it was decided not to make it a "sealed" trait in case additional methods are added in the
/// future that would be implementable by end-user code in a meaningfully varied way.
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
/// [`dec_by`](crate::IntoDecBy::dec_by) is called. Currently, the blanket implementation of this
/// trait exported from the crate for `RangeBounds` is the only possible useful implementation, but
/// it was decided not to make it a "sealed" trait in case additional methods are added in the
/// future that would be implementable by end-user code in a meaningfully varied way.
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
