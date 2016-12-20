pub fn next_pos(v: [i32; 2], cardinal: char, relative: char, steps: i32) -> ([i32; 2], char) {
  let dir = next_dir(cardinal, relative);
  (vec_add(scalar_mult(steps, apply(dir)), v), dir)
}

pub fn get_distance(s: &str) -> i32 {
  let (v, _) = s.split(", ")
    .fold(([0, 0], 'N'), |acc, d| next_pos(acc.0, acc.1, dir(d), to_num(&d[1..])));
  v[0].abs() + v[1].abs()
}

fn to_num(s: &str) -> i32 {
  let out: i32 = s.parse()
    .expect("wanted a number");
  out
}

fn dir(s: &str) -> char {
  s.chars().nth(0).unwrap()
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
