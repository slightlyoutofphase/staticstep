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
fn dec_by_exclusive() {
  let mut r = (64..0).dec_by::<16>();
  assert_eq!(r.next(), Some(64));
  assert_eq!(r.next(), Some(48));
  assert_eq!(r.next(), Some(32));
  assert_eq!(r.next(), Some(16));
  assert_eq!(r.next(), None);
}

#[test]
fn dec_by_inclusive() {
  let mut r = (64..=0).dec_by::<16>();
  assert_eq!(r.next(), Some(64));
  assert_eq!(r.next(), Some(48));
  assert_eq!(r.next(), Some(32));
  assert_eq!(r.next(), Some(16));
  assert_eq!(r.next(), Some(0));
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
  assert_eq!(r.next(), None);
}

#[test]
fn dec_by_inclusive_step_too_big() {
  let mut r = (64..=0).dec_by::<256>();
  assert_eq!(r.next(), None);
}
