use fancy_regex::Regex;
use std::collections::HashSet;
use std::fs;

struct Text {
    content: String,
    content_vec: Vec<char>,
}

impl Text {
    fn new(content: &str) -> Self {
        Text {
            content: String::from(content),
            content_vec: content.chars().collect(),
        }
    }

    fn is_contains_bad_words(&self) -> bool {
        let bad_strings = vec!["ab", "cd", "pq", "xy"];
        for bad in bad_strings.iter() {
            if self.content.find(bad).is_some() {
                return true;
            }
        }
        false
    }

    fn is_contains_vowels(&self) -> bool {
        let mut set: HashSet<char> = HashSet::new();
        let mut count = 0;
        set.insert('a');
        set.insert('e');
        set.insert('i');
        set.insert('o');
        set.insert('u');
        for c in self.content.chars() {
            if set.contains(&c) {
                count += 1;
            }
        }
        count >= 3
    }

    fn is_contains_row(&self) -> bool {
        for i in 0..self.content_vec.len() - 1 {
            if self.content_vec[i] == self.content_vec[i + 1] {
                return true;
            }
        }
        false
    }

    fn is_good(&self) -> bool {
        self.is_contains_row() && self.is_contains_vowels() && !self.is_contains_bad_words()
    }
}

fn get_input(file_path: &str) -> String {
    let content = fs::read_to_string(file_path).expect("Failed to read input");
    content
}

pub fn part_1(input: &str) -> u32 {
    let mut count = 0;
    for l in input.trim().lines() {
        let text = Text::new(l);
        if text.is_good() {
            count += 1;
        }
    }
    count
}

pub fn part_2(input: &str) -> u32 {
    let re_1 = Regex::new(r"(..).*\1").unwrap();
    let re_2 = Regex::new(r"(.).\01").unwrap();

    let mut count = 0;
    for l in input.trim().lines() {
        if re_1.is_match(l).unwrap() && re_2.is_match(l).unwrap() {
            count += 1;
        }
    }
    count
}

pub fn run() {
    let input = get_input("problems/day5/input.txt");
    let part1 = part_1(&input);
    println!("result for part 1: {}", part1);
    let part2 = part_2(&input);
    println!("result for part 2: {}", part2);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1_works() {
        assert_eq!(part_1("ugknbfddgicrmopn"), 1);
        assert_eq!(part_1("aaa"), 1);
        assert_eq!(part_1("jchzalrnumimnmhp"), 0);
        assert_eq!(part_1("haegwjzuvuyypxyu"), 0);
        assert_eq!(part_1("dvszwmarrgswjxmb"), 0);
        assert_eq!(part_1("aaaaaaa"), 1);
        assert_eq!(part_1("aaaaaab"), 0);
        assert_eq!(part_1("acefixx"), 1);
    }
    #[test]
    fn part_2_works() {
        assert_eq!(part_2("qjhvhtzxzqqjkmpb"), 1);
        assert_eq!(part_2("xxyxx"), 1);
        assert_eq!(part_2("uurcxstgmygtbstg"), 0);
        assert_eq!(part_2("ieodomkazucvgmuy"), 0);
    }
}
