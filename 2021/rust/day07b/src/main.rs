fn main() {
    let num_list = include_str!("../input.txt")
        .trim()
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let min_fuel = (0..=*num_list.iter().max().unwrap())
        .map(|n| {
            num_list
                .iter()
                .map(|t| {
                    let d = (n - t).abs();
                    (d * (d + 1)) / 2
                })
                .sum::<i32>()
        })
        .min()
        .unwrap();

    println!("min fuel needed is {}", min_fuel);
}
