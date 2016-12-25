extern crate advent_of_code;

#[cfg(test)]
mod tests {
  use advent_of_code::day4::lib;

  #[test]
  fn gets_id_correctly() {
      assert_eq!(lib::id("aaaaa-bbb-z-y-x-123[abxyz]"), 123);
  }
}
