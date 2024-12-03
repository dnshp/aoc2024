use std::fs;
use regex::Regex;

fn extract_muls(text_segments: Vec<&str>) -> i64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut total: i64 = 0;
    for text in text_segments {
        let captures = re.captures_iter(text);
        for c in captures {
            assert_eq!(c.len(), 3);

            let op1: i64 = c[1].parse().unwrap();
            let op2: i64 = c[2].parse().unwrap();
            total += op1 * op2;
        }
    }
    total
}

fn main() {
    let filename = "dat/input.txt";

    let text: &str = &fs::read_to_string(filename).unwrap()[..];
    /* TODO: part 2 */
    let text_segments: Vec<&str> = vec![text];

    let total: i64 = extract_muls(text_segments);

    println!("{}", total);
}
