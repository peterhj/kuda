use std::cmp::{Ordering};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Version {
  pub major:    u32,
  pub minor:    u32,
  pub patch:    u32,
}

impl PartialOrd for Version {
  fn partial_cmp(&self, other: &Version) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Version {
  fn cmp(&self, other: &Version) -> Ordering {
    match self.major.cmp(&other.major) {
      Ordering::Equal => {}
      Ordering::Greater => return Ordering::Greater,
      Ordering::Less => return Ordering::Less,
    }
    match self.minor.cmp(&other.minor) {
      Ordering::Equal => {}
      Ordering::Greater => return Ordering::Greater,
      Ordering::Less => return Ordering::Less,
    }
    match self.patch.cmp(&other.patch) {
      Ordering::Equal => {}
      Ordering::Greater => return Ordering::Greater,
      Ordering::Less => return Ordering::Less,
    }
    Ordering::Equal
  }
}
