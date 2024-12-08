use std::fs;
use std::error::Error;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

fn parse(file: &str) -> Result<(HashMap<char, Vec<(i32, i32)>>, (i32, i32)), Box<dyn Error>> {
    let frequencies: Vec<char> = [('a'..='z').collect::<Vec::<_>>(), ('A'..='Z').collect::<Vec::<_>>(), ('0'..='9').collect::<Vec::<_>>()].concat();
    let lines = fs::read_to_string(file).unwrap();
    let lines: Vec<Vec<char>> = lines.split("\n").filter(|l| l.len() > 0).map(|l| l.chars().collect()).collect();

    let dims = (lines.len() as i32, lines[0].len() as i32);

    let mut locs: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for f in frequencies {
        locs.insert(f, Vec::new());
    }
    // if we find any characters that are keys to the map, record the locations
    for (i, l) in lines.iter().enumerate() {
        for (j, c) in l.iter().enumerate() {
            match locs.get_mut(&c) {
                Some(v) => v.push((i as i32, j as i32)),
                None => {},
            };
        }
    }

    Ok((locs, dims))
}

fn get_antinodes(a: &(i32, i32), b: &(i32, i32), dimensions: (i32, i32)) -> HashSet<(i32, i32)> {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    let xdiff = b.0 - a.0;
    let ydiff = b.1 - a.1;
    // the "inside" antinodes, ie. those between the two antennas, only exist if the distance
    // between them in x and y is divisible by 3. they will be at 1/3 and 2/3 of the line
    // connecting the antennas.
    if xdiff % 3 == 0 && ydiff % 3 == 0 {
        antinodes.insert((a.0 + (xdiff / 3), a.1 + (ydiff / 3)));
        antinodes.insert((b.0 - (xdiff / 3), b.1 - (ydiff / 3)));
    }
    // the "outside" antinodes, ie. those collinear with the antennas but not between them, exist
    // as long as their locations would be within the dimensions of the space.
    let outside_antinodes = vec![
        (a.0 - xdiff, a.1 - ydiff),
        (b.0 + xdiff, b.1 + ydiff)
    ];
    for candidate in outside_antinodes {
        if candidate.0 >= 0 && candidate.0 < dimensions.0 && candidate.1 >= 0 && candidate.1 < dimensions.1 {
            antinodes.insert(candidate);
        }
    }

    antinodes
}

pub fn main(file: &str) {
    let (locs, dims) = parse(file).unwrap();
    // for each frequency, consider every possible combination of points.
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for (_, antennas) in locs.iter() {
        let antenna_pairs = antennas.iter().combinations(2);
        for pair in antenna_pairs {
            assert_eq!(pair.len(), 2);
            antinodes.extend(get_antinodes(pair[0], pair[1], dims));
        }
    }

    println!("Part 1: {} antinodes", antinodes.len());
}
