use std::fs;

fn get_input(filepath: &str) -> (i32, i32) {
    let content = fs::read_to_string(filepath).expect("Cannnot read input file");
}
