use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::cmp;


fn main() -> io::Result<()> {
    let file = File::open("src/aoc1_input.txt")?;
    let reader = BufReader::new(file);

    let mut curr_max = 0;
    let mut next_max = 0;

    for line in reader.lines() {
        let line_str = line?.to_string();
        if line_str == "" {
            curr_max = cmp::max(curr_max, next_max);
            next_max = 0;
        } else {
            let line_val = line_str.parse::<i32>().unwrap();
            next_max = next_max + line_val;
        }
    }
    println!("{}", curr_max);

    Ok(())
}
