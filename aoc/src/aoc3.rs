use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn priorities(c: char) -> i32 {
    if c.is_lowercase() {
        return (c as u8 - 96) as i32;
    }

    (c as u8 - 64) as i32 + 26
}

fn main() -> io::Result<()> {
    let file = File::open("src/aoc3_input.txt")?;
    let reader = BufReader::new(file);

    let mut total = 0;

    for line in reader.lines() {
        let line_str = line?.to_string();
        let (item_1, item_2) = line_str.split_at(line_str.len() / 2);

        let set_1: HashSet<char> = HashSet::from_iter(item_1.chars());
        let set_2: HashSet<char> = HashSet::from_iter(item_2.chars());

        let common_item = set_1.intersection(&set_2).next();
        // let common_item_value = common_item.unwrap().to_string().parse::<i32>().unwrap();

        total = total + priorities(*common_item.unwrap());
        // println!("{:?} and {:?} and {:?}", set_1, set_2, common_item);
        println!("{}", total)
    }

    Ok(())
}
