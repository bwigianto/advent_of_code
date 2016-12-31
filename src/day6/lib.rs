use std::collections::HashMap;

pub fn zip_combine(a: Vec<HashMap<char, i32>>, b: Vec<HashMap<char, i32>>) -> Vec<HashMap<char, i32>> {
    if b.len() == 0 { return a; }
    if a.len() == 0 { return b; }

    let mut vec = Vec::new();
    for (a1, b1) in a.iter().zip(b.iter()) {
        vec.push(combine(a1, b1));
    }
    vec
}

pub fn combine(a: &HashMap<char, i32>, b: &HashMap<char, i32>) -> HashMap<char, i32> {
    let mut m = HashMap::new();
    for (k, v) in a.iter() {
        if b.contains_key(k) {
            m.insert(*k, *v + b.get(k).unwrap());
        } else {
            m.insert(*k, *v);
        }
    }
    for (k, v) in b.iter() {
        if !m.contains_key(k) {
            m.insert(*k, *v);
        }
    }
    m
}

pub fn vec_map(s: &str) -> Vec<HashMap<char, i32>> {
    s.chars()
        .map(|c| { let mut m = HashMap::new(); m.insert(c, 1); return m;})
        .collect()
}

fn int(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
}
