mod parse_input;

fn part_1(data: &Vec<i32>) -> i32 {
    let mut result = 0;
    for number in data {
        result += get_fuel(*number);
    }
    result
}

fn part_2(data: &Vec<i32>) -> i32 {
    let mut result = 0;
    for number in data {
        let mut current = *number;
        while get_fuel(current) > 0 {
            let next = get_fuel(current);
            result += next;
            current = next;
        }
    }
    result
}

fn get_fuel(fuel: i32) -> i32 {
    fuel / 3 - 2
}

fn main() {
    let filepath = "input.txt";
    let mut input = parse_input::Input {
        filepath,
        data: vec![],
    };
    input.read_data().expect("Failed to load input data");
    let result_1 = part_1(&input.data);
    let result_2 = part_2(&input.data);

    println!("part 1 result is {}", result_1);
    println!("part 2 result is {}", result_2);
}
