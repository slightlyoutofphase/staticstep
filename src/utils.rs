use core::iter::Step;

// We use this specifically to handle the case of `RangeFrom` for `IncBy`.
// Note that the compiler is able to trivially evaluate it down to a constant,
// so it has zero impact on performance.
#[inline(always)]
pub(crate) fn max_value<T: Copy + Default + Step>(_val: &T) -> usize {
  if Step::forward_checked(T::default(), usize::MAX).is_some() {
    usize::MAX
  // Best we can do here currently, as `Step` itself isn't really designed in a way that accounts
  // for `u128` and `i128`.
  } else if Step::forward_checked(T::default(), u128::MAX as usize).is_some() {
    u128::MAX as usize
  } else if Step::forward_checked(T::default(), u64::MAX as usize).is_some() {
    u64::MAX as usize
  } else if Step::forward_checked(T::default(), u32::MAX as usize).is_some() {
    u32::MAX as usize
  } else if Step::forward_checked(T::default(), u16::MAX as usize).is_some() {
    u16::MAX as usize
  } else if Step::forward_checked(T::default(), u8::MAX as usize).is_some() {
    u8::MAX as usize
  } else if Step::forward_checked(T::default(), isize::MAX as usize).is_some() {
    isize::MAX as usize
  // Best we can do here currently, as `Step` itself isn't really designed in a way that accounts
  // for `u128` and `i128`.
  } else if Step::forward_checked(T::default(), i128::MAX as usize).is_some() {
    i128::MAX as usize
  } else if Step::forward_checked(T::default(), i64::MAX as usize).is_some() {
    i64::MAX as usize
  } else if Step::forward_checked(T::default(), i32::MAX as usize).is_some() {
    i32::MAX as usize
  } else if Step::forward_checked(T::default(), i16::MAX as usize).is_some() {
    i16::MAX as usize
  } else if Step::forward_checked(T::default(), i8::MAX as usize).is_some() {
    i8::MAX as usize
  } else {
    // No better way to handle this at the moment. Not a huge deal IMO, as you'd have to be using
    // explicit `RangeFrom` syntax on some kind of struct or enum with a custom `RangeBounds`
    // implementation to get here anyways, and that seems like a really niche / unlikely scenario.
    Default::default()
  }
}
