extern crate advent_of_code;
extern crate itertools;

#[cfg(test)]
mod tests {
    use advent_of_code::day6::lib;
    use std::collections::HashMap;
    use std::io::{BufReader,BufRead};
    use std::fs::File;
    use itertools::Itertools;

    #[test]
    fn str_to_vec_map() {
        let maps = lib::vec_map("asd");
        assert_eq!(maps[0].get(&'a').unwrap(), &1);
        assert_eq!(maps[1].get(&'s').unwrap(), &1);
        assert_eq!(maps[2].get(&'d').unwrap(), &1);
    }

    #[test]
    fn combine_two_maps() {
        let maps = lib::vec_map("as");
        let c = lib::combine(&maps[0], &maps[1]);
        assert_eq!(c.len(), 2);
        assert_eq!(c.get(&'a').unwrap(), &1);
        assert_eq!(c.get(&'s').unwrap(), &1);
    }

    #[test]
    fn conflate_two_maps() {
        let maps = lib::vec_map("a");
        let c = lib::combine(&maps[0], &maps[0]);
        assert_eq!(c.len(), 1);
        assert_eq!(c.get(&'a').unwrap(), &2);
    }

    #[test]
    fn combine_two_vec_of_empty_maps() {
        let maps1 = lib::vec_map("as");
        let two_maps = lib::zip_combine(maps1, Vec::new());
        assert_eq!(two_maps[0].get(&'a').unwrap(), &1);
        assert_eq!(two_maps[1].get(&'s').unwrap(), &1);
    }

    #[test]
    fn combine_two_vec_of_maps() {
        let maps1 = lib::vec_map("as");
        let maps2 = lib::vec_map("af");
        let two_maps = lib::zip_combine(maps1, maps2);
        assert_eq!(two_maps.len(), 2);
        assert_eq!(two_maps[0].len(), 1);
        assert_eq!(two_maps[0].get(&'a').unwrap(), &2);
        assert_eq!(two_maps[1].get(&'s').unwrap(), &1);
        assert_eq!(two_maps[1].get(&'f').unwrap(), &1);
    }

    #[test]
    #[ignore]
    fn part1() {
        let file = File::open("src/day6/input.txt").unwrap();
        let v: String = BufReader::new(file).lines()
            .map(|line| lib::vec_map(&line.unwrap()))
            .fold(Vec::new(), |acc, m| lib::zip_combine(acc, m))
            .iter_mut()
            .map(|m| m.iter()
                .sorted_by(|a, b| b.1.cmp(a.1))
                .iter()
                .map(|&t| t.0)
                .nth(0)
                .unwrap())
            .cloned()
            .collect::<String>();
        println!("{}", v);
    }

    #[test]
    #[ignore]
    fn part2() {
        let file = File::open("src/day6/input.txt").unwrap();
        let v: String = BufReader::new(file).lines()
            .map(|line| lib::vec_map(&line.unwrap()))
            .fold(Vec::new(), |acc, m| lib::zip_combine(acc, m))
            .iter_mut()
            .map(|m| m.iter()
                .sorted_by(|a, b| a.1.cmp(b.1))
                .iter()
                .map(|&t| t.0)
                .nth(0)
                .unwrap())
            .cloned()
            .collect::<String>();
        println!("{}", v);
    }
}
