use std::fs;
use std::error::Error;
use std::num::ParseIntError;
use std::iter::zip;

fn parse_line_to_pair(line: &str) -> Result<(i64, i64), Box<dyn Error>> {
    let pair: Vec<&str> = line.split(" ").filter(|e| e.len() > 0).collect();
    assert_eq!(pair.len(), 2);
    let pair: (Result<i64, ParseIntError>, Result<i64, ParseIntError>) = (pair[0].parse(), pair[1].parse());
    let pair0 = pair.0?;
    let pair1 = pair.1?;

    Ok((pair0, pair1))
}

fn parse_file(name: &str) -> Result<Vec<Vec<i64>>, Box<dyn Error>> {
    let data = fs::read_to_string(name)?;
    let lines = data.split("\n");
    let mut lists: Vec<Vec<i64>> = vec![Vec::new(), Vec::new()];

    for line in lines {
        if line.len() > 0 {
            let pair = parse_line_to_pair(line)?;
            lists[0].push(pair.0);
            lists[1].push(pair.1);
        }
    }

    Ok(lists)
}

fn main() {
    let mut lists = parse_file("dat/input.txt").unwrap();
    lists[0].sort();
    lists[1].sort();
    let pairs = zip(&lists[0], &lists[1]);
    let mut diff: i64 = 0;
    for pair in pairs {
        diff += (pair.0 - pair.1).abs();
    }
    println!("{}", diff)
}
