use std::fs;
extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;

fn get_input(file_path: &str) -> String {
    let content = fs::read_to_string(file_path).expect("Failed to read input");
    content
}
pub fn part_1(input: &str) -> u32 {
    let mut hasher = Md5::new();

    let key = input.trim().as_bytes();
    for i in 0..std::u32::MAX {
        hasher.input(key);
        hasher.input(i.to_string().as_bytes());

        let mut output = [0; 16]; // An MD5 is 16 bytes
        hasher.result(&mut output);

        let first_five = output[0] as i32 + output[1] as i32 + (output[2] >> 4) as i32;
        if first_five == 0 {
            return i;
        }
        hasher.reset();
    }
    0
}

pub fn part_2(input: &str) -> u32 {
    let mut hasher = Md5::new();

    let key = input.trim().as_bytes();
    for i in 0..std::u32::MAX {
        hasher.input(key);
        hasher.input(i.to_string().as_bytes());

        let mut output = [0; 16]; // An MD5 is 16 bytes
        hasher.result(&mut output);

        let first_five = output[0] as i32 + output[1] as i32 + output[2] as i32;
        if first_five == 0 {
            return i;
        }
        hasher.reset();
    }
    0
}

pub fn run() {
    let input = get_input("problems/day4/input.txt");
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
        assert_eq!(part_1("abcdef"), 609043);
        assert_eq!(part_1("pqrstuv"), 1048970);
    }
    #[test]
    fn part_2_works() {
        // assert_eq!(part_2("^v"), 3);
        // assert_eq!(part_2("^>v<"), 3);
        // assert_eq!(part_2("^v^v^v^v^v"), 11);
    }
}
