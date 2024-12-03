use std::fs;
use regex::Regex;

fn main() {
    let filename = "dat/input.txt";

    let text: &str = &fs::read_to_string(filename).unwrap()[..];

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let captures = re.captures_iter(text);

    let mut total: i64 = 0;
    for c in captures {
        assert_eq!(c.len(), 3);

        let op1: i64 = c[1].parse().unwrap();
        let op2: i64 = c[2].parse().unwrap();
        total += op1 * op2;
    }

    println!("{}", total);
}
