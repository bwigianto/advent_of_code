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
  #[ignore]
  fn problem3_1() {
    let file = File::open("src/day3/day3.txt").unwrap();
    let triangles = BufReader::new(file).lines()
      .map(|line| triple(&line.unwrap()))
      .filter(|&tri| lib::valid_triangle(tri))
      .count();
    assert_eq!(triangles, 1050);
  }

  #[test]
  #[ignore]
  fn problem3_2() {
    let file = File::open("src/day3/day3.txt").unwrap();
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();
    let mut v3 = Vec::new();
    for triple in BufReader::new(file).lines()
      .map(|line| triple(&line.unwrap())) {
      v1.push(triple.0);
      v2.push(triple.1);
      v3.push(triple.2);
    }
    v1.append(&mut v2);
    v1.append(&mut v3);
    let triangles = v1
      .chunks(3)
      .map(|x| (x[0], x[1], x[2]))
      .filter(|&tri| lib::valid_triangle(tri))
      .count();
    assert_eq!(triangles, 1921);
  }

  fn triple(line: &str) -> (i32, i32, i32) {
    let mut iter = line.trim().split_whitespace();
    let triple = (int(iter.next().unwrap()), int(iter.next().unwrap()), int(iter.next().unwrap()));
    triple
  }

  fn int(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
  }
}
