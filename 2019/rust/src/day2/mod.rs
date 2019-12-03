use std::fs;
fn get_input(filepath: &'static str) -> Vec<i32> {
    let content = fs::read_to_string(filepath).expect("Cannnot read input file");
    let numbers: Vec<i32> = content
        .split(',')
        .map(|s| s.trim().parse().expect("Failed to parse string into i32"))
        .collect();
    numbers
}

fn compute(data: &mut Vec<i32>) {
    let mut index = 0;
    let mut exit = false;
    let length = data.len();
    while data[index] != 99 && index < length && !exit {
        let operator = data[index];
        let index_1 = data[index + 1];
        let index_2 = data[index + 2];
        let target_index = data[index + 3];
        // println!(
        //     "operator: {}, index_1: {}, index_2: {}, target_index: {}",
        //     operator, index_1, index_2, target_index
        // );
        match operator {
            1 => data[target_index as usize] = data[index_1 as usize] + data[index_2 as usize],
            2 => data[target_index as usize] = data[index_1 as usize] * data[index_2 as usize],

            _ => exit = true,
        }
        index += 4;
    }
}

pub fn part_1() {
    let filepath = "input/day2.txt";
    let mut data = get_input(filepath);
    compute(&mut data);

    data[1] = 12;
    data[2] = 2;

    compute(&mut data);
    println!("{:?}", data)
}

pub fn part_2() {
    let filepath = "input/day2.txt";
    let mut data = get_input(filepath);
    let length = data.len();
    // don't quite like the brute force
    // is there any better ways?
    for x in 1..length {
        for y in 1..length {
            data = get_input(filepath);
            data[1] = x as i32;
            data[2] = y as i32;
            compute(&mut data);
            if data[0] == 19690720 {
                println!("100 * noun + verb: {}", 100 * x + y);
                ()
            }
        }
    }
}
