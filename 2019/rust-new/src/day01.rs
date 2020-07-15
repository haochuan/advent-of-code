use std::io;

pub fn run<T>(mut input: T)
where
    T: io::BufRead,
{
    let mut buffer = String::new();
    let mut total = 0;

    loop {
        if input.read_line(&mut buffer).unwrap() == 0 {
            break;
        }
        let n = buffer.trim().parse::<usize>().unwrap();

        total += match (n / 3).checked_sub(2) {
            Some(i) => i,
            None => 0,
        };

        buffer.clear();
    }
    println!("{}", total);
}
