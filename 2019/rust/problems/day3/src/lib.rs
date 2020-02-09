use std::cmp;
use std::collections::HashSet;
use std::fs;

fn get_input(filepath: &str) -> (Vec<(String, i32)>, Vec<(String, i32)>) {
    let content = fs::read_to_string(filepath).expect("Cannnot read input file");
    let lines: Vec<&str> = content.lines().collect();
    let first: Vec<(String, i32)> = lines[0]
        .split(",")
        .map(|c| {
            // c => R111
            let dir = c[..1].to_string();
            let step: i32 = c[1..].parse().unwrap();
            (dir, step)
        })
        .collect();
    let second: Vec<(String, i32)> = lines[1]
        .split(",")
        .map(|c| {
            // c => R111
            let dir = c[..1].to_string();
            let step: i32 = c[1..].parse().unwrap();
            (dir, step)
        })
        .collect();
    (first, second)
}

fn get_point(command: Vec<(String, i32)>) -> HashSet<(i32, i32)> {
    let mut set: HashSet<(i32, i32)> = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    for c in command {
        let (dir, step) = c;
        let dir = &dir[..];
        for _ in 0..step {
            set.insert((x, y));
            match dir {
                "U" => y = y - 1,
                "D" => y = y + 1,
                "L" => x = x - 1,
                "R" => x = x + 1,
                _ => panic!("Invalid direction"),
            }
        }
    }
    set
}

fn get_intersection(p1: HashSet<(i32, i32)>, p2: HashSet<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut intersection: Vec<(i32, i32)> = vec![];
    for point in &p1 {
        if p2.contains(point) {
            intersection.push(*point);
        }
    }
    intersection
}

fn min_distance(intersection: Vec<(i32, i32)>) -> i32 {
    let mut min = std::i32::MAX;
    for i in &intersection {
        let (x, y) = *i;
        if x != 0 && y != 0 {
            min = cmp::min(min, x.abs() + y.abs());
        }
    }
    min
}

fn part_1() {
    let (first, second) = get_input("problems/day3/input.txt");
    let points_1 = get_point(first);
    let points_2 = get_point(second);
    // println!("{:?}", points_1);
    // println!("{:?}", points_2);

    let intersection = get_intersection(points_1, points_2);
    let min = min_distance(intersection);
    println!("Part 1 is {}", min);
}

pub fn run() {
    part_1();
}
