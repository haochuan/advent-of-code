use crate::error::Error;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io;
use std::ops::Deref;

pub fn run<T>(mut input: T) -> Result<(), Error>
where
    T: io::BufRead,
{
    let mut content = Vec::new();
    input.read_to_end(&mut content)?;

    //part 1
    let mut reader = io::BufReader::new(&content[..]);
    run_part_1(&mut reader)?;

    // part 2
    let mut reader = io::BufReader::new(&content[..]);
    run_part_2(&mut reader)?;
    Ok(())
}

pub fn run_part_1<T>(input: &mut T) -> Result<(), Error>
where
    T: io::BufRead,
{
    let mut buffer = String::new();
    let mut graph = Graph::new();
    loop {
        if input.read_line(&mut buffer)? == 0 {
            break;
        }
        graph.insert_directional(&buffer.trim().to_string())?;
        buffer.clear();
    }
    let connections = graph.count_orbit("COM".to_string())?;
    println!("part 1: {}", connections);
    Ok(())
}
pub fn run_part_2<T>(input: &mut T) -> Result<(), Error>
where
    T: io::BufRead,
{
    let mut buffer = String::new();
    let mut graph = Graph::new();
    loop {
        if input.read_line(&mut buffer)? == 0 {
            break;
        }
        graph.insert_undirectional(&buffer.trim().to_string())?;
        buffer.clear();
    }
    let shortest_distance = graph.count_step("YOU".to_string(), "SAN".to_string())?;
    println!("part 2: {}", shortest_distance);
    Ok(())
}

struct Graph {
    data: HashMap<String, Vec<String>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn insert_directional(&mut self, input: &String) -> Result<(), Error> {
        let Edge(start, end) = Edge::new(input);
        self.data.entry(start).or_insert(Vec::new()).push(end);
        Ok(())
    }
    fn insert_undirectional(&mut self, input: &String) -> Result<(), Error> {
        let Edge(start, end) = Edge::new(input);
        self.data
            .entry(start.clone())
            .or_insert(Vec::new())
            .push(end.clone());
        self.data.entry(end).or_insert(Vec::new()).push(start);
        Ok(())
    }

    fn count_orbit(&self, start: String) -> Result<usize, Error> {
        let mut n_connections = 0;
        let mut visited: HashSet<&str> = HashSet::new();
        let mut levels: HashMap<&str, usize> = HashMap::new();
        let mut queue: VecDeque<&str> = VecDeque::new();
        queue.push_back(&start);
        levels.insert(&start, 0);
        visited.insert(&start);
        while let Some(node) = queue.pop_front() {
            if let Some(childern) = self.data.get(node) {
                for child in childern {
                    if !visited.contains(child.deref()) {
                        // do something
                        let level = levels.get(node).unwrap() + 1;
                        n_connections += level;
                        levels.insert(child, level);
                        visited.insert(child);
                        queue.push_back(child);
                    }
                }
            }
        }
        Ok(n_connections)
    }

    fn count_step(&self, start: String, end: String) -> Result<usize, Error> {
        let mut visited: HashSet<&str> = HashSet::new();
        let mut levels: HashMap<&str, usize> = HashMap::new();
        let mut queue: VecDeque<&str> = VecDeque::new();
        queue.push_back(&start);
        levels.insert(&start, 0);
        visited.insert(&start);
        while let Some(node) = queue.pop_front() {
            if let Some(childern) = self.data.get(node) {
                for child in childern {
                    if !visited.contains(child.deref()) {
                        // do something
                        let level = levels.get(node).unwrap() + 1;
                        if child == &end {
                            return Ok(level - 2); // start and end exclusive
                        }
                        levels.insert(child, level);
                        visited.insert(child);
                        queue.push_back(child);
                    }
                }
            }
        }
        Ok(0)
    }
}

struct Edge(String, String);

impl Edge {
    fn new(input: &String) -> Self {
        let input_vec: Vec<&str> = input.split(")").collect();
        Edge(input_vec[0].to_string(), input_vec[1].to_string())
    }
}
