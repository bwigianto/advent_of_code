extern crate advent_of_code;

#[cfg(test)]
mod tests {
  use advent_of_code::day4::lib;
  use std::io::{BufReader,BufRead};
  use std::fs::File;

  #[test]
  fn gets_id_correctly() {
      assert_eq!(lib::id("aaaaa-bbb-z-y-x-123[abxyz]"), 123);
  }

  #[test]
  fn gets_checksum() {
    assert_eq!(lib::checksum("aaaaa-bbb-z-y-x-123[abxyz]"), "abxyz");
  }

  #[test]
  fn get_code() {
    assert_eq!(lib::code_part("aaaaa-bbb-z-y-x-123[abxyz]"), "aaaaa-bbb-z-y-x");
  }

  #[test]
  fn get_real_checksum() {
    assert_eq!(lib::real_checksum("aaaa-bb-z-y-x"), "abxyz");
  }

  #[test]
  fn get_real_checksum_stops_at_5_chars() {
    assert_eq!(lib::real_checksum("aaaa-bb-z-y-x-c-d-e"), "abcde");
  }

  #[test]
  fn real() {
    assert_eq!(lib::real("aaaa-bbb-z-y-x-123[abxyz]"), true);
  }
  
  #[test]
  fn fake() {
    assert_eq!(lib::real("aaaa-bbb-z-y-x-123[abxyzq]"), false);
  }

  #[test]
  fn shift_char() {
    assert_eq!(lib::shift('a', 1), 'b');
    assert_eq!(lib::shift('b', 2), 'd');
    assert_eq!(lib::shift('y', 2), 'a');
  }

  #[test]
  fn shift_str() {
    assert_eq!(lib::shift_str("xyz", 1), "yza");
  }

  #[test]
  #[ignore]
  fn part1() {
     let file = File::open("src/day4/input.txt").unwrap();
     let ct: i32 = BufReader::new(file).lines()
         .map(|line| line.unwrap())
         .filter(|line| lib::real(&line))
         .map(|line| lib::id(&line))
         .sum();
     assert_eq!(ct, 409147);
  }

  #[test]
  #[ignore]
  fn part2() {
    let file = File::open("src/day4/input.txt").unwrap();
    for line in BufReader::new(file).lines()
      .map(|line| line.unwrap())
      .filter(|line| lib::real(&line)) {
      let code = lib::code_part(&line);
      let id = lib::id(&line);
      let shifted = lib::shift_str(code, id);
      if shifted.contains("north") {
          println!("{}:{}", shifted, id);
      }
    }
  }
}
