use fancy_regex::Regex;
use std::fs;
use std::str;

fn get_input(file_path: &str) -> String {
    let content = fs::read_to_string(file_path).expect("Failed to read input");
    content.trim().to_string()
}

fn increment(input: &mut [u8], position: usize) {
    if input[position] == b'z' {
        input[position] = b'a';
        increment(input, position - 1)
    } else {
        input[position] += 1;
        if input[position] == b'i' || input[position] == b'l' || input[position] == b'o' {
            input[position] += 1;
            for i in position + 1..input.len() {
                input[i] = b'a';
            }
        }
    }
}

fn has_straight(input: &[u8]) -> bool {
    for i in 0..input.len() - 2 {
        if (input[i] == input[i + 1] - 1) && (input[i] == input[i + 2] - 2) {
            return true;
        }
    }
    false
}

fn validate(input: &[u8]) -> bool {
    let re = Regex::new(r"(.)\1.*(.)\2").unwrap();

    if !has_straight(input) {
        return false;
    }

    for i in input.iter() {
        if *i == b'o' || *i == b'i' || *i == b'l' {
            return false;
        }
    }

    let str_form = str::from_utf8(input).expect("");

    if !re.is_match(str_form).unwrap() {
        return false;
    }

    true
}

fn solve(input: String) -> String {
    let len = input.len();
    let mut s = Vec::from(input.as_bytes());

    increment(&mut s, len - 1);

    while !validate(&s) {
        increment(&mut s, len - 1);
    }

    let str_form = str::from_utf8(&s).expect("");
    str_form.to_string()
}

pub fn run() {
    let input = get_input("problems/day11/input.txt");
    let part1 = solve(input);
    println!("result for part 1: {}", part1);
    let part2 = solve(part1);
    println!("result for part 2: {}", part2);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn increment_works_1() {
        let mut s = Vec::from("abc".as_bytes());
        let len = s.len();
        increment(&mut s, len - 1);
        assert_eq!(str::from_utf8(&s).expect(""), "abd");
    }
    #[test]
    fn increment_works_2() {
        let mut s = Vec::from("abz".as_bytes());
        let len = s.len();
        increment(&mut s, len - 1);
        assert_eq!(str::from_utf8(&s).expect(""), "aca");
    }
    #[test]
    fn validate_works_1() {
        let mut a = Vec::from("hijklmmn".as_bytes());
        assert_eq!(validate(&a), false);

        let mut b = Vec::from("abcdffaa".as_bytes());
        assert_eq!(validate(&b), true);
    }
    #[test]
    fn part_2_works() {
        // assert_eq!(part_2("^v"), 3);
        // assert_eq!(part_2("^>v<"), 3);
        // assert_eq!(part_2("^v^v^v^v^v"), 11);
    }
}
