// fn main() {
//     // let matrix: Vec<String> = include_str!("../input.txt")
//     let matrix: Vec<String> = include_str!("../test.txt")
//         .lines()
//         .map(|line| String::from(line))
//         .collect();
//     let result = calculate(&matrix);
//     println!("result: {}", result);
// }

// struct Digit {
//     zero: u32,
//     one: u32,
// }

// impl Digit {
//     fn new() -> Self {
//         Digit { zero: 0, one: 0 }
//     }

//     fn add(&mut self, c: char) {
//         match c {
//             '0' => self.zero += 1,
//             '1' => self.one += 1,
//             _ => (),
//         }
//     }

//     fn get_most_common(&self) -> char {
//         if self.zero > self.one {
//             '0'
//         } else {
//             '1'
//         }
//     }

//     fn get_least_common(&self) -> char {
//         if self.zero > self.one {
//             '1'
//         } else {
//             '0'
//         }
//     }
// }

// enum UsizeOrMatrix {
//     Usize(usize),
//     Matrix(Vec<String>),
// }

// fn get_nums_by_digit<'a>(
//     mut matrix: &'a Vec<String>,
//     bit: char,
//     index: usize,
// ) -> Option<UsizeOrMatrix> {
//     let mut match_list: Vec<String> = vec![];
//     for line in matrix {
//         if line.chars().nth(index).unwrap() == bit {
//             match_list.push(line.to_string());
//         }
//     }

//     println!("match list: {:?}", match_list);

//     if match_list.len() == 1 {
//         Some(UsizeOrMatrix::Usize(
//             usize::from_str_radix(&match_list[0], 2).unwrap(),
//         ))
//     } else {
//         Some(UsizeOrMatrix::Matrix(match_list))
//     }
// }

// fn calculate(matrix: &Vec<String>) -> usize {
//     let mut result: Vec<Digit> = vec![];
//     for list in matrix {
//         list.chars().enumerate().for_each(|(i, c)| {
//             if result.len() != list.len() {
//                 let digit = Digit::new();
//                 result.push(digit);
//             }
//             result[i].add(c);
//         });
//     }

//     let mut oxygen: Option<UsizeOrMatrix> = None;
//     let mut co2: Option<UsizeOrMatrix> = None;

//     let mut matrix_1 = matrix.clone();
//     let mut matrix_2 = matrix.clone();

//     for (i, digit) in result.iter().enumerate() {
//         let most_common = digit.get_most_common();
//         let least_common = digit.get_least_common();

//         if oxygen.is_some() && co2.is_some() {
//             break;
//         } else {
//             match oxygen {
//                 Some(UsizeOrMatrix::Usize(_)) => (),
//                 Some(UsizeOrMatrix::Matrix(new_matrix)) => {
//                     matrix_1 = new_matrix;
//                     oxygen = get_nums_by_digit(&matrix_1, most_common, i)
//                 }
//                 None => (),
//             }
//             match co2 {
//                 Some(UsizeOrMatrix::Usize(_)) => (),
//                 Some(UsizeOrMatrix::Matrix(new_matrix)) => {
//                     matrix_2 = new_matrix;
//                     co2 = get_nums_by_digit(&matrix_2, least_common, i)
//                 }
//                 None => (),
//             }
//         }
//         // println!(
//         //     "calculate at index {}, most common: {}, least common: {}, oxygen: {:?}, co2: {:?}",
//         //     i, most_common, least_common, oxygen, co2
//         // );
//     }
//     return oxygen.unwrap() * co2.unwrap();
// }
#![feature(drain_filter)]

const WIDTH: usize = 12;

pub fn main() {
    // let nums = include_str!("../input.txt")
    let nums = include_str!("../test.txt")
        .lines()
        .map(|l| u32::from_str_radix(l, 2).unwrap())
        .collect::<Vec<_>>();

    let oxy = (0..WIDTH)
        .rev()
        .scan(nums.clone(), |oxy, i| {
            let one = oxy.iter().filter(|n| *n & 1 << i > 0).count() >= (oxy.len() + 1) / 2;
            oxy.drain_filter(|n| (*n & 1 << i > 0) != one);
            oxy.first().copied()
        })
        .last()
        .unwrap();

    let co2 = (0..WIDTH)
        .rev()
        .scan(nums, |co2, i| {
            let one = co2.iter().filter(|n| *n & 1 << i > 0).count() >= (co2.len() + 1) / 2;
            co2.drain_filter(|n| (*n & 1 << i > 0) == one);
            co2.first().copied()
        })
        .last()
        .unwrap();

    println!("{}", oxy * co2);
}
