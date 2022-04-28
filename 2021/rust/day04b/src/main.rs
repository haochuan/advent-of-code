use std::{io::BufRead, vec};

fn main() {
    let mut ops: Vec<i32> = vec![];
    let mut board_list: Vec<Board> = vec![];
    let mut row_list: Vec<Vec<i32>> = vec![];
    // let matrix: Vec<String> = include_str!("../input.txt")
    // include_str!("../test.txt")
    include_str!("../input.txt")
        .lines()
        .enumerate()
        .for_each(|(index, line)| {
            if index == 0 {
                ops = line
                    .split(",")
                    .map(|n| n.trim().parse::<i32>().unwrap())
                    .collect();
            } else if index % 6 == 0 {
                row_list.push(
                    line.split_whitespace()
                        .map(|number| number.trim().parse::<i32>().unwrap())
                        .collect(),
                );
                // println!(
                //     "length: {}, start: {}, end: {}",
                //     row_list.len(),
                //     index - 6,
                //     index - 1
                // );
                board_list.push(Board::new(
                    row_list[5 * (index / 6 - 1)..5 * (index / 6)].to_vec(),
                ));
            } else {
                if line.trim() != "" {
                    row_list.push(
                        line.split_whitespace()
                            .map(|number| number.trim().parse::<i32>().unwrap())
                            .collect(),
                    );
                }
            }
        });
    let mut result: i32 = 0;
    for op in ops {
        for board in &mut board_list {
            board.mark(op);
            if !board.did_win() && board.has_win() {
                board.mark_win();
                result = board.get_unmarked_sum() * op;
            }
        }
    }
    println!("result is {}.", result);
}

struct Point {
    number: i32,
    marked: bool,
}

impl From<i32> for Point {
    fn from(number: i32) -> Self {
        Point {
            number,
            marked: false,
        }
    }
}

impl Point {
    fn new(number: i32) -> Self {
        Point {
            number,
            marked: false,
        }
    }

    fn mark(&mut self) {
        self.marked = true;
    }

    fn is_marked(&self) -> bool {
        self.marked
    }
}

struct Board {
    content: Vec<Vec<Point>>,
    // Track how many has been mark in a col/row
    // 0-4 rows, 5-9 cols
    // [0..10], (0,0) -> (0, 0 + 5), (1, 2) -> (1, 2 + 5)
    point_map: Vec<i32>,
    is_win: bool,
}

impl Board {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let content: Vec<Vec<Point>> = matrix
            .iter()
            .map(|row| row.iter().map(|number| Point::from(*number)).collect())
            .collect();
        Board {
            content,
            point_map: vec![0; 10],
            is_win: false,
        }
    }

    fn mark(&mut self, number: i32) {
        for i in 0..5 {
            let row = &mut self.content[i];
            for j in 0..5 {
                let p = &mut row[j];
                if p.number == number {
                    p.mark();
                    self.point_map[i] += 1;
                    self.point_map[j + 5] += 1;
                }
            }
        }
    }

    fn has_win(&self) -> bool {
        for item in &self.point_map {
            if *item == 5 {
                return true;
            }
        }
        false
    }

    fn get_unmarked_sum(&self) -> i32 {
        let mut sum = 0;
        for i in 0..5 {
            let row = &self.content[i];
            for j in 0..5 {
                let p = &row[j];
                if !p.is_marked() {
                    sum += p.number;
                }
            }
        }
        sum
    }

    fn mark_win(&mut self) {
        self.is_win = true;
    }

    fn did_win(&self) -> bool {
        self.is_win
    }
}
