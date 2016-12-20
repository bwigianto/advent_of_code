extern crate advent_of_code;

#[cfg(test)]
mod tests {
  use advent_of_code::day1::lib;

  #[test]
  fn one_step() {
    assert_eq!(lib::next_pos([0, 0], 'N', 'R', 4), ([4, 0], 'E'));
  }

  #[test]
  fn one_other_step() {
    assert_eq!(lib::next_pos([0, 0], 'N', 'L', 4), ([-4, 0], 'W'));
  }

  #[test]
  fn one_step_from_text() {
    assert_eq!(lib::get_distance("R4"), 4);
  }

  #[test]
  fn two_steps_from_text() {
    assert_eq!(lib::get_distance("R4, R6"), 10);
  }

  #[test]
  fn one_step_forward_one_back() {
    assert_eq!(lib::get_distance("R4, L4, L4, L4"), 0);
  }

  #[test]
  fn problem1() {
    assert_eq!(lib::get_distance("R1, R1, R3, R1, R1, L2, R5, L2, R5, R1, R4, L2, R3, L3, R4, L5, R4, R4, R1, L5, L4, R5, R3, L1, R4, R3, L2, L1, R3, L4, R3, L2, R5, R190, R3, R5, L5, L1, R54, L3, L4, L1, R4, R1, R3, L1, L1, R2, L2, R2, R5, L3, R4, R76, L3, R4, R191, R5, R5, L5, L4, L5, L3, R1, R3, R2, L2, L2, L4, L5, L4, R5, R4, R4, R2, R3, R4, L3, L2, R5, R3, L2, L1, R2, L3, R2, L1, L1, R1, L3, R5, L5, L1, L2, R5, R3, L3, R3, R5, R2, R5, R5, L5, L5, R2, L3, L5, L2, L1, R2, R2, L2, R2, L3, L2, R3, L5, R4, L4, L5, R3, L4, R1, R3, R2, R4, L2, L3, R2, L5, R5, R4, L2, R4, L1, L3, L1, L3, R1, R2, R1, L5, R5, R3, L3, L3, L2, R4, R2, L5, L1, L1, L5, L4, L1, L1, R1"), 241);
  }
}
