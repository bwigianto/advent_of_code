extern crate advent_of_code;

#[cfg(test)]
mod tests {
  use advent_of_code::day4::lib;

  #[test]
  fn gets_id_correctly() {
      assert_eq!(lib::id("aaaaa-bbb-z-y-x-123[abxyz]"), 123);
  }

  #[test]
  fn gets_id_correctly2() {
      assert_eq!(lib::id("aaaaa-y-x-3214[blah]"), 3214);
  }

  #[test]
  fn gets_checksum() {
      assert_eq!(lib::checksum("aaaaa-bbb-z-y-x-123[abxyz]"), "abxyz");
  }
}
