use staticstep::*;

#[test]
fn inc_by_exclusive() {
  let mut r = (0..64).inc_by::<16>();
  assert_eq!(r.next(), Some(0));
  assert_eq!(r.next(), Some(16));
  assert_eq!(r.next(), Some(32));
  assert_eq!(r.next(), Some(48));
  assert_eq!(r.next(), None);
}

#[test]
fn inc_by_exclusive_unbound_start() {
  let mut r = (..64).inc_by::<16>();
  assert_eq!(r.next(), Some(0));
  assert_eq!(r.next(), Some(16));
  assert_eq!(r.next(), Some(32));
  assert_eq!(r.next(), Some(48));
  assert_eq!(r.next(), None);
}

#[test]
fn inc_by_inclusive() {
  let mut r = (0..=64).inc_by::<16>();
  assert_eq!(r.next(), Some(0));
  assert_eq!(r.next(), Some(16));
  assert_eq!(r.next(), Some(32));
  assert_eq!(r.next(), Some(48));
  assert_eq!(r.next(), Some(64));
  assert_eq!(r.next(), None);
}

#[test]
fn inc_by_inclusive_unbound_start() {
  let mut r = (..=64).inc_by::<16>();
  assert_eq!(r.next(), Some(0));
  assert_eq!(r.next(), Some(16));
  assert_eq!(r.next(), Some(32));
  assert_eq!(r.next(), Some(48));
  assert_eq!(r.next(), Some(64));
  assert_eq!(r.next(), None);
}

#[test]
fn dec_by_exclusive_usize() {
  let mut r = (64usize..0usize).dec_by::<16>();
  assert_eq!(r.next(), Some(64));
  assert_eq!(r.next(), Some(48));
  assert_eq!(r.next(), Some(32));
  assert_eq!(r.next(), Some(16));
  assert_eq!(r.next(), None);
}

#[test]
fn dec_by_inclusive_usize() {
  let mut r = (64usize..=0usize).dec_by::<16>();
  assert_eq!(r.next(), Some(64));
  assert_eq!(r.next(), Some(48));
  assert_eq!(r.next(), Some(32));
  assert_eq!(r.next(), Some(16));
  assert_eq!(r.next(), Some(0));
  assert_eq!(r.next(), None);
}

#[test]
fn dec_by_exclusive_isize() {
  let mut r = (64isize..0isize).dec_by::<16>();
  assert_eq!(r.next(), Some(64));
  assert_eq!(r.next(), Some(48));
  assert_eq!(r.next(), Some(32));
  assert_eq!(r.next(), Some(16));
  assert_eq!(r.next(), None);
}

#[test]
fn dec_by_inclusive_isize() {
  let mut r = (64isize..=0isize).dec_by::<16>();
  assert_eq!(r.next(), Some(64));
  assert_eq!(r.next(), Some(48));
  assert_eq!(r.next(), Some(32));
  assert_eq!(r.next(), Some(16));
  assert_eq!(r.next(), Some(0));
  assert_eq!(r.next(), None);
}

#[test]
fn dec_by_exclusive_unbound_end() {
  let mut r = (64isize..).dec_by::<16>();
  assert_eq!(r.next(), Some(64));
  assert_eq!(r.next(), Some(48));
  assert_eq!(r.next(), Some(32));
  assert_eq!(r.next(), Some(16));
  assert_eq!(r.next(), None);
}

#[test]
fn inc_by_exclusive_wrong() {
  let mut r = (64..0).inc_by::<16>();
  assert_eq!(r.next(), None);
}

#[test]
fn inc_by_inclusive_wrong() {
  let mut r = (64..=0).inc_by::<16>();
  assert_eq!(r.next(), None);
}

#[test]
fn dec_by_exclusive_wrong() {
  let mut r = (0..64).dec_by::<16>();
  assert_eq!(r.next(), None);
}

#[test]
fn dec_by_inclusive_wrong() {
  let mut r = (0..=64).dec_by::<16>();
  assert_eq!(r.next(), None);
}

#[test]
fn inc_by_exclusive_step_too_big() {
  let mut r = (0..64).inc_by::<256>();
  assert_eq!(r.next(), Some(0));
  assert_eq!(r.next(), None);
}

#[test]
fn inc_by_inclusive_step_too_big() {
  let mut r = (0..=64).inc_by::<256>();
  assert_eq!(r.next(), Some(0));
  assert_eq!(r.next(), None);
}

#[test]
fn dec_by_exclusive_step_too_big() {
  let mut r = (64..0).dec_by::<256>();
  assert_eq!(r.next(), Some(64));
  assert_eq!(r.next(), None);
}

#[test]
fn dec_by_inclusive_step_too_big() {
  let mut r = (64..=0).dec_by::<256>();
  assert_eq!(r.next(), Some(64));
  assert_eq!(r.next(), None);
}

#[test]
fn dec_by_exclusive_negative_end_even_step() {
  let mut r = (64isize..-64isize).dec_by::<16>();
  assert_eq!(r.next(), Some(64));
  assert_eq!(r.next(), Some(48));
  assert_eq!(r.next(), Some(32));
  assert_eq!(r.next(), Some(16));
  assert_eq!(r.next(), Some(0));
  assert_eq!(r.next(), Some(-16));
  assert_eq!(r.next(), Some(-32));
  assert_eq!(r.next(), Some(-48));
  assert_eq!(r.next(), None);
}

#[test]
fn dec_by_inclusive_negative_end_even_step() {
  let mut r = (64isize..=-64isize).dec_by::<16>();
  assert_eq!(r.next(), Some(64));
  assert_eq!(r.next(), Some(48));
  assert_eq!(r.next(), Some(32));
  assert_eq!(r.next(), Some(16));
  assert_eq!(r.next(), Some(0));
  assert_eq!(r.next(), Some(-16));
  assert_eq!(r.next(), Some(-32));
  assert_eq!(r.next(), Some(-48));
  assert_eq!(r.next(), Some(-64));
  assert_eq!(r.next(), None);
}

#[test]
fn dec_by_exclusive_negative_end_uneven_step() {
  // This should be the same as the inclusive version below, based on how many times
  // the value of `STEP` fits between `self.start` and `self.end` in this case.
  let mut r = (64isize..-64isize).dec_by::<27>();
  assert_eq!(r.next(), Some(64));
  assert_eq!(r.next(), Some(37));
  assert_eq!(r.next(), Some(10));
  assert_eq!(r.next(), Some(-17));
  assert_eq!(r.next(), Some(-44));
  assert_eq!(r.next(), None);
}

#[test]
fn dec_by_inclusive_negative_end_uneven_step() {
  let mut r = (64isize..=-64isize).dec_by::<27>();
  assert_eq!(r.next(), Some(64));
  assert_eq!(r.next(), Some(37));
  assert_eq!(r.next(), Some(10));
  assert_eq!(r.next(), Some(-17));
  assert_eq!(r.next(), Some(-44));
  assert_eq!(r.next(), None);
}

#[test]
fn inc_by_exclusive_char_range() {
  let mut r = ('A'..'G').inc_by::<2>();
  assert_eq!(r.next(), Some('A'));
  assert_eq!(r.next(), Some('C'));
  assert_eq!(r.next(), Some('E'));
  assert_eq!(r.next(), None);
}

#[test]
fn inc_by_inclusive_char_range() {
  let mut r = ('A'..='G').inc_by::<2>();
  assert_eq!(r.next(), Some('A'));
  assert_eq!(r.next(), Some('C'));
  assert_eq!(r.next(), Some('E'));
  assert_eq!(r.next(), Some('G'));
  assert_eq!(r.next(), None);
}

#[test]
fn dec_by_exclusive_char_range() {
  let mut r = ('G'..'A').dec_by::<2>();
  assert_eq!(r.next(), Some('G'));
  assert_eq!(r.next(), Some('E'));
  assert_eq!(r.next(), Some('C'));
  assert_eq!(r.next(), None);
}

#[test]
fn dec_by_inclusive_char_range() {
  let mut r = ('G'..='A').dec_by::<2>();
  assert_eq!(r.next(), Some('G'));
  assert_eq!(r.next(), Some('E'));
  assert_eq!(r.next(), Some('C'));
  assert_eq!(r.next(), Some('A'));
  assert_eq!(r.next(), None);
}

#[test]
fn inc_by_type_dependent_overflow() {
  let mut r = (248u8..255u8).inc_by::<50>();
  assert_eq!(r.next(), None);
  let mut r2 = (248u32..255u32).inc_by::<{ usize::MAX }>();
  assert_eq!(r2.next(), None);
  let mut r3 = (248i32..255i32).inc_by::<{ u32::MAX as usize }>();
  assert_eq!(r3.next(), None);
  // We already check this in multiple other tests, but below we look
  // to see if only overflowing the *range* as opposed to overflowing
  // some specific type still returns `Some` at least once, which it should.
  let mut r4 = (248..255).inc_by::<50>();
  assert_eq!(r4.next(), Some(248));
  assert_eq!(r4.next(), None);
  let mut r5 = (248usize..255usize).inc_by::<{ u32::MAX as usize }>();
  assert_eq!(r5.next(), Some(248));
  assert_eq!(r5.next(), None);
}
