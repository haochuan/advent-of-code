use std::fs;

fn get_input(file_path: &str) -> String {
    let content = fs::read_to_string(file_path).expect("Failed to read input");
    content
}

pub fn part_1(input: &str) -> i32 {
    let mut counter = 0;
    for c in input.chars() {
        if c == '(' {
            counter += 1;
        } else if c == ')' {
            counter -= 1;
        }
    }
    counter
}

pub fn part_2(input: &str) -> usize {
    let mut counter = 0;
    for (i, c) in input.chars().enumerate() {
        if c == '(' {
            counter += 1;
        } else if c == ')' {
            counter -= 1;
        }
        if counter == -1 {
            return i + 1;
        }
    }
    0
}

pub fn run() {
    let input = get_input("problems/day1/input.txt");
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
        assert_eq!(part_1("((("), 3);
        assert_eq!(part_1("(())"), 0);
        assert_eq!(part_1(")())())"), -3);
    }
    #[test]
    fn part_2_works() {
        assert_eq!(part_2(")"), 1);
        assert_eq!(part_2("()())"), 5);
    }
}
