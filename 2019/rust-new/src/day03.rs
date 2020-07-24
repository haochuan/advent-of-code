use crate::error::Error;
use std::collections::HashMap;
use std::io;

struct Grid {
    current_distance: i32,
    current_pos: (i32, i32),
    point_hash: HashMap<(i32, i32), i32>, // just for first line
    intersection: Vec<(i32, i32)>,
}

impl Grid {
    fn new() -> Self {
        Self {
            current_distance: 0,
            current_pos: (0, 0),
            point_hash: HashMap::new(),
            intersection: Vec::new(),
        }
    }

    fn reset_current_pos(&mut self) {
        self.current_pos = (0, 0);
        self.current_distance = 0;
    }

    fn add_point(&mut self, is_first: bool) {
        let (x, y) = self.current_pos;
        if is_first {
            // first line
            self.point_hash.insert((x, y), self.current_distance);
        } else {
            // second line
            if self.point_hash.contains_key(&(x, y)) {
                self.intersection.push((x, y));
                *self.point_hash.get_mut(&(x, y)).unwrap() += self.current_distance;
            }
        }
    }
    fn add_line(&mut self, instruction: (&str, i32), is_first: bool) {
        let (dir, step) = instruction;
        let mut x_inc: i32 = 0;
        let mut y_inc: i32 = 0;
        match dir {
            "L" => x_inc = -1,
            "R" => x_inc = 1,
            "U" => y_inc = -1,
            "D" => y_inc = 1,
            _ => (),
        }

        let (mut x_curr, mut y_curr) = self.current_pos;
        for _ in 1..=step {
            x_curr += x_inc;
            y_curr += y_inc;
            self.current_distance += 1;
            self.current_pos = (x_curr, y_curr);
            self.add_point(is_first);
        }
    }
}

pub fn run<T>(mut input: T) -> Result<(), Error>
where
    T: io::BufRead,
{
    let mut content = Vec::new();
    input.read_to_end(&mut content)?;

    let mut reader = io::BufReader::new(&content[..]);
    run_part_all(&mut reader)?;

    Ok(())
}

pub fn run_part_all<T>(input: &mut T) -> Result<(), Error>
where
    T: io::BufRead,
{
    let mut buffer = String::new();
    let mut first_line = true;

    let mut grid = Grid::new();
    loop {
        if input.read_line(&mut buffer)? == 0 {
            break;
        }
        let data: Vec<&str> = buffer.trim().split(",").collect();
        for code in data {
            let dir = &code[..1];
            let step: i32 = code[1..].parse().unwrap();
            grid.add_line((dir, step), first_line);
        }

        first_line = false;
        grid.reset_current_pos();
        buffer.clear();
    }

    let mut result_1 = i32::MAX;
    let mut result_2 = i32::MAX;
    // println!("{:?}", grid.intersection);
    for (x, y) in grid.intersection {
        let current = x.abs() + y.abs();
        if current < result_1 {
            result_1 = current;
        }

        let steps = match grid.point_hash.get(&(x, y)) {
            Some(&s) => s,
            None => 0,
        };

        if steps < result_2 {
            result_2 = steps;
        }
    }
    println!("part 1: {}", result_1);
    println!("part 2: {}", result_2);
    Ok(())
}
