use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Input {
    pub filepath: &'static str,
    pub data: Vec<i32>,
}

impl Input {
    pub fn read_data(&mut self) -> Result<(), Box<dyn Error>> {
        let file = File::open(self.filepath)?;
        let reader = BufReader::new(file);
        for (_, line) in reader.lines().enumerate() {
            let line = line?;
            let number: i32 = line.parse()?;
            self.data.push(number);
        }
        Ok(())
    }
}
