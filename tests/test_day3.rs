extern crate advent_of_code;

#[cfg(test)]
mod tests {
  use advent_of_code::day3::lib;
  use std::io::{BufReader,BufRead};
  use std::fs::File; 

  #[test]
  fn triangle() {
    assert_eq!(lib::valid_triangle((3, 4, 5)), true);
  }

  #[test]
  fn not_triangle() {
    assert_eq!(lib::valid_triangle((3, 4, 7)), false);
  }

  #[test]
  fn problem3_1() {
    let file = File::open("src/day3/day3.txt").unwrap();
    let triangles = BufReader::new(file).lines()
      .map(|line| triple(&line.unwrap()))
      .filter(|&tri| lib::valid_triangle(tri))
      .count();
    assert_eq!(triangles, 1050);
  }

  #[test]
  fn problem3_2() {
    let file = File::open("src/day3/day3.txt").unwrap();
    let triples = BufReader::new(file).lines()
      .map(|line| triple(&line.unwrap()))
      .collect();
    triples.foo();
    // transpose(triples)
    //   .filter(|&tri| lib::valid_triangle(tri))
    //   .count();
    // assert_eq!(triangles, 1050);
  }

  // fn transpose()

  fn triple(line: &str) -> (i32, i32, i32) {
    let mut iter = line.trim().split_whitespace();
    let triple = (int(iter.next().unwrap()), int(iter.next().unwrap()), int(iter.next().unwrap()));
    triple
  }

  fn int(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
  }
}