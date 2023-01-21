use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;


fn main() -> io::Result<()> {
    let file = File::open("src/aoc2_input.txt")?;
    let reader = BufReader::new(file);

    let win_map = HashMap::from([
        ("A", "Y"),
        ("B", "Z"),
        ("C", "X"),
    ]);

    let draw_map = HashMap::from([
        ("A", "X"),
        ("B", "Y"),
        ("C", "Z"),
    ]);

    let lose_map = HashMap::from([
        ("A", "Z"),
        ("B", "X"),
        ("C", "Y"),
    ]);

    let map_map = HashMap::from([
        ("X", lose_map),
        ("Y", draw_map),
        ("Z", win_map),
    ]);

    let move_map = HashMap::from([
        ("X", 1),
        ("Y", 2),
        ("Z", 3),
    ]);

    let result_map = HashMap::from([
        ("X", 0),
        ("Y", 3),
        ("Z", 6),
    ]);


    let mut total_points = 0;

    for line in reader.lines() {
        let line_elems = line.unwrap();
        let mut spit_str = line_elems.split_whitespace();
        let opp_move = spit_str.next().unwrap();
        let outcome = spit_str.next().unwrap();

        total_points = total_points + result_map[outcome];
        total_points = total_points + move_map[map_map[outcome][opp_move]];
        println!("{}", total_points);
    }
    println!("{}", total_points);

    Ok(())
}
