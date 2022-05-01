fn main() {
    let mut fish_list: Vec<Fish> = include_str!("../input.txt")
        .trim()
        // include_str!("../input.txt")
        .split(",")
        .map(|n| Fish::new(n.parse::<u8>().unwrap()))
        .collect();
    for _ in 0..80 {
        let mut new_to_add = 0;
        fish_list.iter_mut().for_each(|fish| {
            let should_create = fish.next_day();
            if should_create {
                new_to_add += 1;
            }
        });

        for _ in 0..new_to_add {
            fish_list.push(Fish::new(8));
        }
    }
    println!("After 80 days there are {} fish.", fish_list.len());
}

struct Fish {
    state: u8,
}

impl Fish {
    fn new(state: u8) -> Self {
        Fish { state }
    }

    fn next_day(&mut self) -> bool {
        // return true if it resets
        if self.state == 0 {
            self.state = 6;
            true
        } else {
            self.state -= 1;
            false
        }
    }
}
