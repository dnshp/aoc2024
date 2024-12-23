use std::fs;
use std::collections::HashSet;

fn find_reachable_summits(map: &Vec<Vec<u8>>, cache: &mut Vec<Vec<Option<usize>>>, pos: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut summits: HashSet<(usize, usize)> = HashSet::new();
    let val = map[pos.0][pos.1];

    if val == 9 {
        cache[pos.0][pos.1] = Some(1);
        summits.insert(pos);
    } else {
        if pos.0 > 0 && map[pos.0 - 1][pos.1] == val + 1 {
            summits.extend(find_reachable_summits(map, cache, (pos.0 - 1, pos.1)));
        }
        if pos.0 < map.len() - 1 && map[pos.0 + 1][pos.1] == val + 1 {
            summits.extend(find_reachable_summits(map, cache, (pos.0 + 1, pos.1)));
        }
        if pos.1 > 0 && map[pos.0][pos.1 - 1] == val + 1 {
            summits.extend(find_reachable_summits(map, cache, (pos.0, pos.1 - 1)));
        }
        if pos.1 < map[0].len() - 1 && map[pos.0][pos.1 + 1] == val + 1 {
            summits.extend(find_reachable_summits(map, cache, (pos.0, pos.1 + 1)));
        }

        cache[pos.0][pos.1] = Some(summits.len());
    }
    summits
}

fn find_summit_paths(map: &Vec<Vec<u8>>, cache: &mut Vec<Vec<Option<usize>>>, pos: (usize, usize)) -> HashSet<Vec<(usize, usize)>> {
    let mut paths: HashSet<Vec<(usize, usize)>> = HashSet::new();
    let val = map[pos.0][pos.1];

    if val == 9 {
        cache[pos.0][pos.1] = Some(1);
        paths.insert(vec![pos]);
    } else {
        if pos.0 > 0 && map[pos.0 - 1][pos.1] == val + 1 {
            for mut p in find_summit_paths(map, cache, (pos.0 - 1, pos.1)) {
                p.insert(0, pos);
                paths.insert(p);
            }
        }
        if pos.0 < map.len() - 1 && map[pos.0 + 1][pos.1] == val + 1 {
            for mut p in find_summit_paths(map, cache, (pos.0 + 1, pos.1)) {
                p.insert(0, pos);
                paths.insert(p);
            }
        }
        if pos.1 > 0 && map[pos.0][pos.1 - 1] == val + 1 {
            for mut p in find_summit_paths(map, cache, (pos.0, pos.1 - 1)) {
                p.insert(0, pos);
                paths.insert(p);
            }
        }
        if pos.1 < map[0].len() - 1 && map[pos.0][pos.1 + 1] == val + 1 {
            for mut p in find_summit_paths(map, cache, (pos.0, pos.1 + 1)) {
                p.insert(0, pos);
                paths.insert(p);
            }
        }

        cache[pos.0][pos.1] = Some(paths.len());
    }
    paths
}

pub fn main(file: &str) {
    let map: Vec<Vec<u8>> = fs::read_to_string(file).unwrap()
                                                    .split("\n")
                                                    .filter(|l| l.len() > 0)
                                                    .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
                                                    .collect();
    let mut cache: Vec<Vec<Option<usize>>> = Vec::new();
    for _i in 0..map.len() {
        let mut tmp: Vec<Option<usize>> = Vec::new();
        for _j in 0..map[0].len() {
            tmp.push(None);
        }
        cache.push(tmp);
    }
    let mut cache2 = cache.clone();

    // part 1
    let mut total = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 0 {
                match cache[i][j] {
                    Some(v) => {total += v;},
                    None => {total += find_reachable_summits(&map, &mut cache, (i, j)).len();}
                };
                // no cache, for testing.
                // let summits = find_reachable_summits(&map, &mut cache, (i, j)).len();
                // total += summits;
            }
        }
    }
    println!("part 1: {}", total);

    // part 2
    let mut total = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 0 {
                match cache2[i][j] {
                    Some(v) => {total += v;},
                    None => {total += find_summit_paths(&map, &mut cache2, (i, j)).len();}
                }
            }
        }
    }
    println!("part 2: {}", total);
}
