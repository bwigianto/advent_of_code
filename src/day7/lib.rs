use std::collections::HashMap;
use regex::Regex;
use regex::FindCaptures;

pub fn ssl(s: &str) -> bool {
    let supernets: Vec<String> = unbrackets(s).iter()
        .flat_map(|ub| subs_of_size(&ub, 3))
        .filter(|tri| aba(tri))
        .map(|s| s.to_string())
        .collect();
    if supernets.len() == 0 { return false; }
    let hypernets: Vec<String> = brackets(s).iter()
        .flat_map(|ub| subs_of_size(&ub, 3))
        .filter(|tri| aba(tri))
        .map(|s| s.to_string())
        .collect();
    for tri in supernets {
        if hypernets.contains(&inverse(&tri)) {
            return true;
        }
    }
    return false;
}

pub fn tls(s: &str) -> bool {
    let hypernet_abba = brackets(s).iter()
        .flat_map(|ub| subs_of_size(&ub, 4))
        .any(|word| anagram(word));
    if hypernet_abba { return false; }
    unbrackets(s).iter()
        .flat_map(|ub| subs_of_size(&ub, 4))
        .any(|word| anagram(word))
}

pub fn subs_of_size(s: &str, n: i32) -> Vec<&str> {
    if s.len() < n as usize { return Vec::new(); }
    (0..(s.len() - (n as usize) + 1))
        .map(|i| &s[i..(i + (n as usize))])
        .collect()
}

pub fn inverse(s: &str) -> String {
    let cs: Vec<char> = s.chars().collect();
    [cs[1], cs[0], cs[1]].iter().cloned().collect()
}

pub fn aba(s: &str) -> bool {
    let v = s.as_bytes();
    if v[0] == v[1] { return false;}
    v[0] == v[2]
}

pub fn anagram(s: &str) -> bool {
    let v = s.as_bytes();
    if v[0] == v[1] { return false;}
    if (v[0] == v[3]) && (v[1] == v[2]) { return true;}
    return false;
}

pub fn unbrackets(s: &str) -> Vec<&str> {
    let re = Regex::new("(?:^|\\])([^\\[]+)(?:\\[|$)").unwrap();
    re.captures_iter(s)
        .map(|s| s.at(1).unwrap())
        .collect()
}

pub fn brackets(s: &str) -> Vec<&str> {
    let re = Regex::new("\\[([^\\]]+)\\]").unwrap();
    re.captures_iter(s)
        .map(|s| s.at(1).unwrap())
        .collect()
}

fn capture(re: Regex, s: &str) -> &str {
     re.captures(s).unwrap().at(1).unwrap()
}
