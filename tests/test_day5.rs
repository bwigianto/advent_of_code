extern crate advent_of_code;

#[cfg(test)]
mod tests {
  use advent_of_code::day5::lib;
  use std::collections::BTreeMap;

  #[test]
  fn cat_numbers_to_str() {
      let base = "ojvtpuvg";
      assert_eq!(lib::cat(base, 2), "ojvtpuvg2");
  }

  #[test]
  fn get_password() {
      let base = "abc";
      assert_eq!(lib::password(base, 5017308), (5017308 as u64, "8f".to_string()));
  }

  #[test]
  #[ignore]
  fn part1() {
      let base = "ojvtpuvg";
      let mut start = 0 as u64;
      let mut pwd = String::new();
      for _ in 0..8 {
          let num_and_char = lib::password(base, start);
          pwd = pwd + &num_and_char.1.to_string();
          start = num_and_char.0 + 1;
      }
      println!("{}", pwd);
  }

  #[test]
  #[ignore]
  fn part2() {
      let base = "ojvtpuvg";
      let mut start = 0 as u64;
      let mut map: BTreeMap<i32, String> = BTreeMap::new();
      while map.len() != 8 {
          let num_and_index_and_char = lib::password(base, start);
          let c = &num_and_index_and_char.1[0..1];
          let ch = &num_and_index_and_char.1[1..2];
          if c == "0" || c == "1" || c == "2" || c == "3" || c == "4" || c == "5" || c == "6" || c == "7"  {
              if !map.contains_key(&int(c)) {
                  map.insert(int(c), ch.to_string());
              }
          }
          start = num_and_index_and_char.0 + 1;
      }
      for (k, v) in map {
          println!("{}:{}", k, v);
      }   
  }
  
  fn int(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
  }
}
