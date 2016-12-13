fn to_num(s: &str) -> i32 {
  let out: i32 = s.parse()
    .expect("wanted a number");
  out
}

fn dir(s: &str) -> char {
  s.chars().nth(0).unwrap()
}

pub fn get_distance(s: &str) -> i32 {
  let (v, c) = s.split(", ")
    .fold(([0, 0], 'N'), |acc, d| next_pos(acc.0, acc.1, dir(d), to_num(&d[1..])));
  v[0].abs() + v[1].abs()
}

fn next_dir(cardinal: char, relative: char) -> char {
  match (cardinal, relative) {
    ('N', 'R') | ('S', 'L') => 'E',
    ('N', 'L') | ('S', 'R') => 'W',
    ('E', 'R') | ('W', 'L') => 'S',
    ('E', 'L') | ('W', 'R') => 'N',
    _ => ' '
  }
}

fn apply(cardinal: char) -> [i32; 2] {
  match cardinal {
    'E' => [1, 0],
    'W' => [-1, 0],
    'S' => [0, -1],
    'N' => [0, 1],
    _ => [0, 0]
  }
}

fn scalar_mult(a: i32, v: [i32; 2]) -> [i32; 2] {
  [a * v[0], a * v[1]]
}

fn vec_add(v: [i32; 2], u: [i32; 2]) -> [i32; 2] {
  [v[0] + u[0], v[1] + u[1]]
}

pub fn next_pos(v: [i32; 2], cardinal: char, relative: char, steps: i32) -> ([i32; 2], char) {
  let dir = next_dir(cardinal, relative);
  (vec_add(scalar_mult(steps, apply(dir)), v), dir)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_step() {
    assert_eq!(next_pos([0, 0], 'N', 'R', 4), ([4, 0], 'E'));
  }

  #[test]
  fn one_other_step() {
    assert_eq!(next_pos([0, 0], 'N', 'L', 4), ([-4, 0], 'W'));
  }

  #[test]
  fn one_step_from_text() {
    assert_eq!(get_distance("R4"), 4);
  }

  #[test]
  fn two_steps_from_text() {
    assert_eq!(get_distance("R4, R6"), 10);
  }

  #[test]
  fn one_step_forward_one_back() {
    assert_eq!(get_distance("R4, L4, L4, L4"), 0);
  }

  #[test]
  fn problem1() {
    assert_eq!(get_distance("R1, R1, R3, R1, R1, L2, R5, L2, R5, R1, R4, L2, R3, L3, R4, L5, R4, R4, R1, L5, L4, R5, R3, L1, R4, R3, L2, L1, R3, L4, R3, L2, R5, R190, R3, R5, L5, L1, R54, L3, L4, L1, R4, R1, R3, L1, L1, R2, L2, R2, R5, L3, R4, R76, L3, R4, R191, R5, R5, L5, L4, L5, L3, R1, R3, R2, L2, L2, L4, L5, L4, R5, R4, R4, R2, R3, R4, L3, L2, R5, R3, L2, L1, R2, L3, R2, L1, L1, R1, L3, R5, L5, L1, L2, R5, R3, L3, R3, R5, R2, R5, R5, L5, L5, R2, L3, L5, L2, L1, R2, R2, L2, R2, L3, L2, R3, L5, R4, L4, L5, R3, L4, R1, R3, R2, R4, L2, L3, R2, L5, R5, R4, L2, R4, L1, L3, L1, L3, R1, R2, R1, L5, R5, R3, L3, L3, L2, R4, R2, L5, L1, L1, L5, L4, L1, L1, R1"), 241);
  }
}
