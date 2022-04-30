fn main() {
    let mut map = vec![0; 1000 * 1000];
    let mut overlaps = 0;
    // include_str!("../test.txt")
    include_str!("../input.txt")
        .lines()
        .map(|line| {
            let ((x1, y1), (x2, y2)) = parse_line(line);
            if x1 == x2 || y1 == y2 {
                (x1.min(x2), y1.min(y2), x1.max(x2), y1.max(y2))
            } else {
                (x1, y1, x2, y2)
            }
        })
        .for_each(|(x1, y1, x2, y2)| {
            let mut mark = |x, y| {
                if map[x + y * 1000] == 1 {
                    overlaps += 1;
                }
                map[x + y * 1000] += 1;
            };

            if x1 == x2 {
                (y1..=y2).for_each(|y| mark(x1, y));
            } else if y1 == y2 {
                (x1..=x2).for_each(|x| mark(x, y1));
            } else {
                // diagonal line at 45 degree
                for step in 0..=(x1 as i32 - x2 as i32).abs() {
                    let step = step as usize;
                    if x1 > x2 {
                        if y1 > y2 {
                            mark(x1 - step, y1 - step);
                        } else {
                            mark(x1 - step, y1 + step);
                        }
                    } else {
                        if y1 > y2 {
                            mark(x1 + step, y1 - step);
                        } else {
                            mark(x1 + step, y1 + step);
                        }
                    }
                }
            }
        });
    println!("overlaps: {}", overlaps);
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
