use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::cmp;
use std::collections::BinaryHeap;


fn main() -> io::Result<()> {
    let file = File::open("src/aoc1_input.txt")?;
    let reader = BufReader::new(file);

    let mut curr_max = 0;
    let mut all_totals = BinaryHeap::new();
    let mut vec: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line_str = line?.to_string();
        if line_str == "" {
            all_totals.push(curr_max);
            vec.push(curr_max);
            curr_max = 0;
        } else {
            let line_val = line_str.parse::<i32>().unwrap();
            curr_max = curr_max + line_val;
        }
    }
    let max_three = all_totals.pop().unwrap() + all_totals.pop().unwrap() + all_totals.pop().unwrap();
    println!("{}", max_three);
    vec.sort();
    println!("{:?}", vec);

    Ok(())
}
