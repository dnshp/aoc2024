use std::fs;
use regex::Regex;
use std::error::Error;
use aho_corasick::{AhoCorasick, PatternID};

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

fn get_valid_segments(text: &str) -> Result<Vec<&str>, Box<dyn Error>> {
    let patterns = &["do()", "don't()"];
    let ac = AhoCorasick::new(patterns)?;
    let matches = ac.find_iter(text);

    let mut valid_segments: Vec<&str> = Vec::new();
    let mut in_valid_segment = true;
    let mut offset = 0;

    for m in matches {
        if in_valid_segment && m.pattern() == PatternID::must(1) {
            valid_segments.push(&text[offset..m.end()]);
            offset = m.end();
            in_valid_segment = false;
        } else if !in_valid_segment && m.pattern() == PatternID::must(0) {
            offset = m.end();
            in_valid_segment = true;
        }
    }
    if in_valid_segment {
        valid_segments.push(&text[offset..]);
    }
    Ok(valid_segments)
}

fn main() {
    let filename = "dat/input.txt";

    let text: &str = &fs::read_to_string(filename).unwrap()[..];
    // let text_segments: Vec<&str> = vec![text]; /* Part 1 */
    let text_segments = get_valid_segments(text).unwrap(); /* Part 2 */

    let total: i64 = extract_muls(text_segments);

    println!("{}", total);
}
