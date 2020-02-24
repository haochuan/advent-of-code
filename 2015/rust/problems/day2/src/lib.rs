use std::cmp::max;
use std::fs;

#[derive(Debug)]
struct Present {
    l: u32,
    w: u32,
    h: u32,
}

impl Present {
    fn new(l: u32, w: u32, h: u32) -> Self {
        Present { l, w, h }
    }

    fn get_slack(&self) -> u32 {
        let largest = max(max(self.l, self.h), self.w);
        if largest == self.l {
            self.h * self.w
        } else if largest == self.h {
            self.l * self.w
        } else {
            self.h * self.l
        }
    }
    fn get_wrap(&self) -> u32 {
        let largest = max(max(self.l, self.h), self.w);
        if largest == self.l {
            (self.h + self.w) * 2
        } else if largest == self.h {
            (self.l + self.w) * 2
        } else {
            (self.l + self.h) * 2
        }
    }

    pub fn get_usage(&self) -> u32 {
        2 * self.l * self.w + 2 * self.l * self.h + 2 * self.w * self.h + self.get_slack()
    }

    pub fn ribbon_usage(&self) -> u32 {
        self.l * self.w * self.h + self.get_wrap()
    }
}

fn create_present(s: &str) -> Present {
    let dimensions: Vec<u32> = s.split('x').map(|s| s.parse::<u32>().unwrap()).collect();
    Present::new(dimensions[0], dimensions[1], dimensions[2])
}

fn get_input(file_path: &str) -> String {
    let content = fs::read_to_string(file_path).expect("Failed to read input");
    content
}

pub fn part_1(input: &str) -> u32 {
    let mut count: u32 = 0;
    for l in input.lines() {
        let p = create_present(l);
        count += p.get_usage();
    }
    count
}

pub fn part_2(input: &str) -> u32 {
    let mut count: u32 = 0;
    for l in input.lines() {
        let p = create_present(l);
        count += p.ribbon_usage();
    }
    count
}

pub fn run() {
    let input = get_input("problems/day2/input.txt");
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
        assert_eq!(part_1("2x3x4"), 58);
        assert_eq!(part_1("1x1x10"), 43);
    }
    #[test]
    fn part_2_works() {
        assert_eq!(part_2("2x3x4"), 34);
        assert_eq!(part_2("1x1x10"), 14);
        assert_eq!(part_2("20x3x11"), 660 + 28);
        assert_eq!(part_2("15x27x5"), 2109);
        assert_eq!(part_2("14x12x8"), 1384);
    }
}
