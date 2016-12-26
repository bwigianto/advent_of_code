use regex::Regex;
use std::collections::HashMap;

pub fn shift_str(s: &str, n: i32) -> String {
  s.chars()
    .fold(String::new(), |res, c| res + &format!("{}", shift(c, n)))
}

pub fn shift(c: char, n: i32) -> char {
  if c == '-' { return '-'; }
  let alpha = "abcdefghijklmnopqrstuvwxyz";
  let index = (alpha.chars().position(|ch| ch == c).unwrap() + n as usize) % alpha.len();
  alpha.chars().nth(index).unwrap()
}

pub fn real(code: &str) -> bool {
    real_checksum(code_part(code)) == checksum(code)
}

pub fn real_checksum(code: &str) -> String {
    let mut count: HashMap<char, u32> = HashMap::new();
    for c in code.chars() {
        if c == '-' { continue; }
        *count.entry(c).or_insert(0) += 1;
    }
    let mut count_vec: Vec<(&char, &u32)> = count.iter().collect();
    count_vec.sort_by(|a, b| {
        if a.1 == b.1 {
            return a.0.cmp(b.0);
        }
        b.1.cmp(a.1)
    });
    count_vec.iter()
        .take(5)
        .map(|e| e.0)
        .fold(String::new(), |res, c| res + &format!("{}", c))
}

pub fn code_part(code: &str) -> &str {
    let re = Regex::new("(.*)-\\d+\\[.*\\]$").unwrap();
    capture(re, code)
}

pub fn checksum(code: &str) -> &str {
    let re = Regex::new(".*\\[(.*)\\]").unwrap();
    capture(re, code)
}

pub fn id(code: &str) -> i32 {
    let re = Regex::new(".*-(\\d+)\\[.*").unwrap();
    int(capture(re, code))
}

fn capture(re: Regex, s: &str) -> &str {
    re.captures(s).unwrap().at(1).unwrap()
}

fn int(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
}
