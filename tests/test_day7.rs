extern crate advent_of_code;
extern crate itertools;
extern crate regex;

#[cfg(test)]
mod tests {
    use advent_of_code::day7::lib;
    use std::io::{BufReader,BufRead};
    use std::fs::File;

    #[test]
    fn aba_tests() {
        assert_eq!(lib::aba("aba"), true);
        assert_eq!(lib::aba("abc"), false);
    }

    #[test]
    fn inverse_tests() {
        assert_eq!(lib::inverse("aba"), "bab");
        assert_eq!(lib::inverse("cbc"), "bcb");
    }

    #[test]
    fn gets_square_bracketed_strs() {
        let b = lib::brackets("abba[mnop]qrst[foo]asjdkf[bar]sdf");
        assert_eq!(b[0], "mnop");
        assert_eq!(b[1], "foo");
        assert_eq!(b[2], "bar");
    }

    #[test]
    fn is_valid_anagram_success() {
        assert_eq!(lib::anagram("abba"), true);
    }

    #[test]
    fn is_valid_anagram_fail() {
        assert_eq!(lib::anagram("aaaa"), false);
        assert_eq!(lib::anagram("abcd"), false);
    }

    #[test]
    fn gets_unbracketed_strs() {
        let ub = lib::unbrackets("abba[mnp]qrst");
        assert_eq!(ub[0], "abba");
        assert_eq!(ub[1], "qrst");
    }

    #[test]
    fn get_substrings_of_size_4_is_empty_if_string_too_short() {
        let substrs = lib::subs_of_size("abc", 4);
        assert_eq!(substrs.len(), 0);
    }

    #[test]
    fn get_substrings_of_size_4() {
        let substrs = lib::subs_of_size("abcdefgh", 4);
        assert_eq!(substrs[0], "abcd");
        assert_eq!(substrs[1], "bcde");
        assert_eq!(substrs[2], "cdef");
        assert_eq!(substrs[3], "defg");
        assert_eq!(substrs[4], "efgh");
    }

    #[test]
    fn supports_tls() {
        assert_eq!(lib::tls("abcd[bddb]xyyx"), false);
        assert_eq!(lib::tls("qabba[mnop]qrst"), true);
        assert_eq!(lib::tls("qabba[mnop]qrst[cddc]"), false);
        assert_eq!(lib::tls("abbc[mnop]qrst[cdde]"), false);
        assert_eq!(lib::tls("dnwtsgywerfamfv[gwrhdujbiowtcirq]bjbhmuxdcasenlctwgh"), false);
    }

    #[test]
    fn supports_ssl() {
        assert_eq!(lib::ssl("aba[bab]xyz"), true);
        assert_eq!(lib::ssl("xyx[xyx]xyx"), false);
    }

    #[test]
    #[ignore]
    fn part1() {
        let file = File::open("src/day7/input.txt").unwrap();
        let s = BufReader::new(file).lines()
            .map(|line| lib::tls(&line.unwrap()))
            .filter(|&ans| ans)
            .count();
        println!("{}", s);
    }

    #[test]
    #[ignore]
    fn part2() {
        let file = File::open("src/day7/input.txt").unwrap();
        let s = BufReader::new(file).lines()
            .map(|line| lib::ssl(&line.unwrap()))
            .filter(|&ans| ans)
            .count();
        println!("{}", s);
    }
}
