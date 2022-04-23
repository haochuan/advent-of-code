fn main() {
    let matrix: Vec<String> = include_str!("../input.txt")
        // let matrix: Vec<String> = include_str!("../test.txt")
        .lines()
        .map(|line| String::from(line))
        .collect();
    let gamma = calculate(&matrix, "gamma");
    let epsilon = calculate(&matrix, "epsilon");
    println!(
        "gamma: {}, epsilon: {}, result: {}",
        gamma,
        epsilon,
        gamma * epsilon
    );
}

struct Digit {
    zero: u32,
    one: u32,
}

impl Digit {
    fn new() -> Self {
        Digit { zero: 0, one: 0 }
    }

    fn add(&mut self, c: char) {
        match c {
            '0' => self.zero += 1,
            '1' => self.one += 1,
            _ => (),
        }
    }

    fn get_most_common(&self) -> &str {
        if self.zero > self.one {
            "0"
        } else {
            "1"
        }
    }

    fn get_least_common(&self) -> &str {
        if self.zero > self.one {
            "1"
        } else {
            "0"
        }
    }
}

fn calculate(matrix: &Vec<String>, kind: &str) -> usize {
    let mut result: Vec<Digit> = vec![];
    for list in matrix {
        list.chars().enumerate().for_each(|(i, c)| {
            if result.len() != list.len() {
                let digit = Digit::new();
                result.push(digit);
            }
            result[i].add(c);
        });
    }

    let binary_list: Vec<&str> = match kind {
        "gamma" => result.iter().map(|digit| digit.get_most_common()).collect(),
        "epsilon" => result
            .iter()
            .map(|digit| digit.get_least_common())
            .collect(),
        _ => vec![],
    };
    usize::from_str_radix(&binary_list.join(""), 2).unwrap()
}
