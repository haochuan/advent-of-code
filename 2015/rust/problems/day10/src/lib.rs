use std::char;
use std::fs;

fn get_input(file_path: &str) -> String {
    let content = fs::read_to_string(file_path).expect("Failed to read input");
    content.trim().to_string()
}

fn generate_next(input: &String) -> String {
    let mut result = String::new();
    let input_vec: Vec<char> = input.chars().collect();
    let mut index = 0;
    let mut cache: Vec<char> = Vec::new();
    while index < input_vec.len() {
        // println!("cache: {:?}", cache);
        if cache.len() == 0 {
            cache.push(input_vec[index]);
        } else {
            let curr_char = cache[0];
            if input_vec[index] != curr_char {
                result.push(char::from_digit(cache.len() as u32, 10).unwrap());
                result.push(curr_char);
                cache.clear();
            }
            cache.push(input_vec[index]);
        }
        index += 1;
    }
    // clean cache
    if cache.len() != 0 {
        result.push(char::from_digit(cache.len() as u32, 10).unwrap());
        result.push(cache[0]);
    }
    result
}

fn generator(input: &String, times: u32) -> usize {
    let mut result = input.to_string();
    for _ in 0..times {
        result = generate_next(&result);
        // println!("{}", result);
    }
    result.len()
}

pub fn run() {
    let input = get_input("problems/day10/input.txt");
    let part1 = generator(&input, 40);
    println!("result for part 1: {}", part1);
    let part2 = generator(&input, 50);
    println!("result for part 2: {}", part2);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generator_1_works() {
        assert_eq!(generator(&"1".to_string(), 5), "312211".to_string());
    }
    #[test]
    fn part_2_works() {
        // assert_eq!(part_2("^v"), 3);
        // assert_eq!(part_2("^>v<"), 3);
        // assert_eq!(part_2("^v^v^v^v^v"), 11);
    }
}
