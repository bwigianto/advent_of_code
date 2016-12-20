pub fn valid_triangle(triangle: (i32, i32, i32)) -> bool {
  match triangle {
    (a, b, c) if a + b <= c => false,
    (a, b, c) if a + c <= b => false,
    (a, b, c) if b + c <= a => false,
    _ => true
  }
}
