use std::fs;
use std::collections::HashMap;

fn iterate(stone: u64, cache: &mut HashMap<(u64, usize), usize>, iters: usize) -> usize {
    if iters == 0 {
        return 1;
    }

    // check if we've encountered this before
    match cache.get(&(stone, iters)) {
        Some(v) => {return *v;},
        None => {},
    };

    let mut i: usize = 0;
    let mut stones: Vec<u64> = vec![stone];

    // do a single iteration
    while i < stones.len() {
        if stones[i] == 0 {
            stones[i] = 1;
            i += 1;
        } else {
            let num_digits = ((stones[i] as f64).log10() as u32) + 1;
            if num_digits % 2 == 0 {
                let divisor = 10_u64.pow(num_digits / 2);
                let upper = stones[i] / divisor;
                let lower = stones[i] % divisor;
                stones[i] = upper;
                stones.insert(i + 1, lower);
                i += 2;
            } else {
                stones[i] *= 2024;
                i += 1;
            }
        }
    }
    
    let total = stones.iter().map(|s| iterate(*s, cache, iters - 1)).sum();
    cache.insert((stone, iters), total);
    total
}

pub fn main(file: &str) {
    // part 1
    let stones: Vec<u64> = fs::read_to_string(file).unwrap().trim().split(" ").map(|n| n.parse().unwrap()).collect();
    let mut cache: HashMap<(u64, usize), usize> = HashMap::new();
    let mut len = 0;
    for s in stones {
        len += iterate(s, &mut cache, 25);
    }
    println!("part 1: {}", len);

    // part 2
    let stones: Vec<u64> = fs::read_to_string(file).unwrap().trim().split(" ").map(|n| n.parse().unwrap()).collect();
    let mut cache: HashMap<(u64, usize), usize> = HashMap::new();
    let mut len = 0;
    for s in stones {
        len += iterate(s, &mut cache, 75);
    }
    println!("part 2: {}", len);
}
