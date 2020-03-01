use std::fs;

fn cal_char(input: &str) -> u32 {
    input.to_string().len() as u32
}

fn cal_memory(input: &str) -> u32 {
    let mut count = 0;
    let data = input.as_bytes();
    // first and last chars are ", so skip
    let mut index: usize = 1;
    while index < data.len() - 1 {
        let first = data[index];
        let second = data[index + 1];
        if first == b'\\' {
            if second == b'x' {
                // \x0f
                index += 3;
            } else {
                index += 1;
            }
        }
        count += 1;
        index += 1;
    }
    count
}

fn cal_encoding_increase(input: &str) -> u32 {
    // adding "", +2
    // first and last are ", will become \", +2
    let mut count = 4;
    let data: Vec<char> = input.chars().collect();
    for i in 0..data.len() {
        let curr = data[i];
        if curr == '"' || curr == '\\' {
            count += 1;
        }
    }
    count
}

fn get_input(file_path: &str) -> String {
    let content = fs::read_to_string(file_path).expect("Failed to read input");
    content
}

pub fn part_1(input: &str) -> u32 {
    let mut char_count = 0;
    let mut memory_count = 0;
    for l in input.trim().lines() {
        char_count += cal_char(l);
        memory_count += cal_memory(l);
    }
    char_count - memory_count
}

pub fn part_2(input: &str) -> u32 {
    let mut encoding_count = 0;
    for l in input.trim().lines() {
        encoding_count += cal_encoding_increase(l);
    }
    encoding_count
}

pub fn run() {
    let input = get_input("problems/day8/input.txt");
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
        let mut char_count = 0;
        let mut memory_count = 0;
        let input = vec![r#"\"\""#, r#"\"abc\""#, r#"\"aaa\"aaa\""#, r#"\"\x27\""#];
        let test = "test";
        let b = test.as_bytes();
        println!("{:?}", b);
        for l in input {
            char_count += cal_char(l);
            memory_count += cal_memory(l);
            println!(
                "input: {}, char: {}, memory: {}",
                l,
                cal_char(l),
                cal_memory(l)
            );
        }
        assert_eq!(char_count - memory_count, 12);
    }
    #[test]
    fn part_2_works() {
        let mut encoding_count = 0;
        let input = vec![r#""#, r#"abc"#, r#"aaa\"aaa"#, r#"\x27"#];
        for l in input {
            encoding_count += cal_encoding_increase(l);
            println!(
                "input: {}, char: {}, encoding: {}",
                l,
                cal_char(l),
                cal_encoding_increase(l)
            );
        }
        assert_eq!(encoding_count, 19);
    }
}
