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
fn inc_by_unbound_end_u8() {
  let mut r = (0u8..).inc_by::<16>();
  assert_eq!(r.next(), Some(0));
  assert_eq!(r.next(), Some(16));
  assert_eq!(r.next(), Some(32));
  assert_eq!(r.next(), Some(48));
  assert_eq!(r.next(), Some(64));
  assert_eq!(r.next(), Some(80));
  assert_eq!(r.next(), Some(96));
  assert_eq!(r.next(), Some(112));
  assert_eq!(r.next(), Some(128));
  assert_eq!(r.next(), Some(144));
  assert_eq!(r.next(), Some(160));
  assert_eq!(r.next(), Some(176));
  assert_eq!(r.next(), Some(192));
  assert_eq!(r.next(), Some(208));
  assert_eq!(r.next(), Some(224));
  assert_eq!(r.next(), Some(240));
}

#[test]
fn inc_by_unbound_end_usize() {
  let mut r = (0usize..).inc_by::<16>();
  assert_eq!(r.next(), Some(0));
  assert_eq!(r.next(), Some(16));
  assert_eq!(r.next(), Some(32));
  assert_eq!(r.next(), Some(48));
  assert_eq!(r.next(), Some(64));
  assert_eq!(r.next(), Some(80));
  assert_eq!(r.next(), Some(96));
  assert_eq!(r.next(), Some(112));
  assert_eq!(r.next(), Some(128));
  assert_eq!(r.next(), Some(144));
  assert_eq!(r.next(), Some(160));
  assert_eq!(r.next(), Some(176));
  assert_eq!(r.next(), Some(192));
  assert_eq!(r.next(), Some(208));
  assert_eq!(r.next(), Some(224));
  assert_eq!(r.next(), Some(240));
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
fn inc_by_exclusive_type_dependent_overflow() {
  let mut r1 = (248u8..255u8).inc_by::<50>();
  assert_eq!(r1.next(), Some(248));
  assert_eq!(r1.next(), None);
  let mut r2 = (248u32..255u32).inc_by::<{ usize::MAX }>();
  assert_eq!(r2.next(), Some(248));
  assert_eq!(r2.next(), None);
  let mut r3 = (248i32..255i32).inc_by::<{ u32::MAX as usize }>();
  assert_eq!(r3.next(), Some(248));
  assert_eq!(r3.next(), None);
}

#[test]
fn inc_by_inclusive_type_dependent_overflow() {
  let mut r4 = (248u8..=255u8).inc_by::<50>();
  assert_eq!(r4.next(), Some(248));
  assert_eq!(r4.next(), None);
  let mut r5 = (248u32..=255u32).inc_by::<{ usize::MAX }>();
  assert_eq!(r5.next(), Some(248));
  assert_eq!(r5.next(), None);
  let mut r6 = (248i32..=255i32).inc_by::<{ u32::MAX as usize }>();
  assert_eq!(r6.next(), Some(248));
  assert_eq!(r6.next(), None);
}

#[test]
fn dec_by_exclusive_type_dependent_overflow() {
  let mut r7 = (255u8..248u8).dec_by::<50>();
  assert_eq!(r7.next(), Some(255));
  assert_eq!(r7.next(), None);
  let mut r8 = (255u32..248u32).dec_by::<{ usize::MAX }>();
  assert_eq!(r8.next(), Some(255));
  assert_eq!(r8.next(), None);
  let mut r9 = (255i32..248i32).dec_by::<{ u32::MAX as usize }>();
  assert_eq!(r9.next(), Some(255));
  assert_eq!(r9.next(), None);
}

#[test]
fn dec_by_inclusive_type_dependent_overflow() {
  let mut r10 = (255u8..=248u8).dec_by::<50>();
  assert_eq!(r10.next(), Some(255));
  assert_eq!(r10.next(), None);
  let mut r11 = (255u32..=248u32).dec_by::<{ usize::MAX }>();
  assert_eq!(r11.next(), Some(255));
  assert_eq!(r11.next(), None);
  let mut r12 = (255i32..=248i32).dec_by::<{ u32::MAX as usize }>();
  assert_eq!(r12.next(), Some(255));
  assert_eq!(r12.next(), None);
}
