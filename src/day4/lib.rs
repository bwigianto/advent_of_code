extern crate regex;

use regex::Regex;

pub fn id(code: &str) -> i32 {
    let re = Regex::new(".*-(\d+)\[.*").unwrap();
    return int(re.captures(code).unwrap());
}

fn int(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
}
