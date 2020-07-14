use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::u64;

fn solve(input: &Vec<String>) -> (u64, u64) {
    // parse the input
    let mut distances: HashMap<(&str, &str), u64> = HashMap::new();
    let mut cities: HashSet<&str> = HashSet::new();

    let re = Regex::new(r"^(.+?) to (.+?) = (\d+?)$").unwrap();
    for line in input.iter() {
        match re.captures(line) {
            Some(cap) => {
                let pair = (cap.get(1).unwrap().as_str(), cap.get(2).unwrap().as_str());
                let distance = cap.get(3).unwrap().as_str().parse().unwrap();
                // the distance between two cities is the same in both directions, so we record
                // it twice.
                distances.insert(pair, distance);
                distances.insert((pair.1, pair.0), distance);
                cities.insert(pair.0);
                cities.insert(pair.1);
                // hack to make the code shorter. The starting point is '#', and it's 0 distance
                // from all cities.
                distances.insert(("#", pair.0), 0);
                distances.insert(("#", pair.1), 0);
            }
            None => {
                panic!("invalid input: {}", line);
            }
        }
    }

    // find the shortest path by recursively searching every possible path
    return solve_rec(&cities, &distances, "#", HashSet::new());
}

fn solve_rec(
    cities: &HashSet<&str>,
    distances: &HashMap<(&str, &str), u64>,
    current: &str,
    visited: HashSet<&str>,
) -> (u64, u64) {
    if visited.len() == cities.len() {
        return (0, 0);
    }
    let mut min = u64::MAX;
    let mut max = 0;
    for city in cities.iter() {
        if visited.contains(city) {
            continue;
        }
        let mut visited = visited.clone();
        visited.insert(*city);
        let t = solve_rec(&cities, &distances, city, visited);
        let t_min = t.0 + distances.get(&(current, *city)).unwrap();
        if t_min < min {
            min = t_min;
        }
        let t_max = t.1 + distances.get(&(current, *city)).unwrap();
        if t_max > max {
            max = t_max;
        }
    }
    return (min, max);
}
fn get_input(file_path: &str) -> Vec<String> {
    let content: Vec<String> = fs::read_to_string(file_path)
        .expect("Failed to read input")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();
    content
}

pub fn run() {
    let input = get_input("problems/day9/input.txt");
    let part1 = solve(&input).0;
    println!("result for part 1: {}", part1);
    let part2 = solve(&input).1;
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
