use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;


fn main() -> io::Result<()> {
    let file = File::open("src/aoc2_input.txt")?;
    let reader = BufReader::new(file);

    let move_map = HashMap::from([
        ("A", "X"),
        ("B", "Y"),
        ("C", "Z"),
    ]);

    let win_map = HashMap::from([
        ("A", "Y"),
        ("B", "Z"),
        ("C", "X"),
    ]);

    let points_map = HashMap::from([
        ("X", 1),
        ("Y", 2),
        ("Z", 3),
    ]);

    let mut total_points = 0;

    for line in reader.lines() {
        let line_elems = line.unwrap();
        let mut spit_str = line_elems.split_whitespace();
        let opp_move = spit_str.next().unwrap();
        let my_move = spit_str.next().unwrap();

        total_points = total_points + points_map[my_move];
        if my_move == move_map[opp_move] {
            total_points = total_points + 3;
        } else if my_move == win_map[opp_move] {
            total_points = total_points + 6;
        }
        println!("{}", total_points);
    }
    println!("{}", total_points);

    Ok(())
}
