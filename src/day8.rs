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
    let xrange = 0..dimensions.0;
    let yrange = 0..dimensions.1;
    let xdiff = b.0 - a.0;
    let ydiff = b.1 - a.1;

    // the "outside" antinodes, ie. those collinear with the antennas but not between them, exist
    // as long as their locations would be within the dimensions of the space.
    let outside_antinodes = vec![
        (a.0 - xdiff, a.1 - ydiff),
        (b.0 + xdiff, b.1 + ydiff)
    ];
    for candidate in outside_antinodes {
        if xrange.contains(&candidate.0) && yrange.contains(&candidate.1) {
            antinodes.insert(candidate);
        }
    }

    antinodes
}

fn get_antinodes_2(a: &(i32, i32), b: &(i32, i32), dimensions: (i32, i32)) -> HashSet<(i32, i32)> {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    // trivially, each antenna is an antinode.
    antinodes.insert(a.clone());
    antinodes.insert(b.clone());

    let xrange = 0..dimensions.0;
    let yrange = 0..dimensions.1;
    let xdiff = b.0 - a.0;
    let ydiff = b.1 - a.1;
    
    // scan "forward"
    let mut candidate = vec![a.0 - xdiff, a.1 - ydiff];
    while xrange.contains(&candidate[0]) && yrange.contains(&candidate[1]) {
        antinodes.insert((candidate[0], candidate[1]));
        candidate[0] -= xdiff;
        candidate[1] -= ydiff;
    }

    // scan "backward"
    let mut candidate = vec![b.0 + xdiff, b.1 + ydiff];
    while xrange.contains(&candidate[0]) && yrange.contains(&candidate[1]) {
        antinodes.insert((candidate[0], candidate[1]));
        candidate[0] += xdiff;
        candidate[1] += ydiff;
    }

    antinodes
}

pub fn main(file: &str) {
    let (locs, dims) = parse(file).unwrap();
    // for each frequency, consider every possible combination of points.
    let mut antinodes: Vec<HashSet<(i32, i32)>> = vec![HashSet::new(), HashSet::new()];
    for (_, antennas) in locs.iter() {
        let antenna_pairs = antennas.iter().combinations(2);
        for pair in antenna_pairs {
            assert_eq!(pair.len(), 2);
            antinodes[0].extend(get_antinodes(pair[0], pair[1], dims));
            antinodes[1].extend(get_antinodes_2(pair[0], pair[1], dims));
        }
    }

    println!("Part 1: {} antinodes", antinodes[0].len());
    println!("Part 2: {} antinodes", antinodes[1].len());
}
