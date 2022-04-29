fn main() {
    // let mut map = vec![0, 1000 * 1000];
    // let mut overlaps = 0;
    include_str!("../test.txt")
        .lines()
        .map(|line| {
            let ((x1, y1), (x2, y2)) = parse_line(line);
            (x1.min(x2), y1.min(y2), x1.max(x2), y1.max(y2))
        })
        .for_each(|(x1, y1, x2, y2)| {
            println!("{}, {}, {}, {}", x1, y1, x2, y2);
        });
}

fn parse_line(input: &str) -> ((usize, usize), (usize, usize)) {
    // 3,4 -> 1,3
    let coord_list: Vec<(usize, usize)> = input
        .split(" -> ")
        .map(|s| {
            let l: Vec<usize> = s.split(",").map(|n| n.parse::<usize>().unwrap()).collect();
            (l[0], l[1])
        })
        .collect();
    (coord_list[0], coord_list[1])
}
