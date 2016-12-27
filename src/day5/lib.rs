use crypto::md5::Md5;
use crypto::digest::Digest;

pub fn password(s: &str, n: u64) -> (u64, String) {
    let mut i = n;
    loop {
        let hash = md5(&cat(s, i));
        if hash.starts_with("00000") {
            return (i, hash[5..7].to_string());
        }
        i = i + 1;
    }
}

pub fn cat(s: &str, n: u64) -> String {
    s.to_string() + &n.to_string()
}

pub fn md5(s: &str) -> String {
    let mut hasher = Md5::new();
    hasher.input_str(&s);
    hasher.result_str()
}
