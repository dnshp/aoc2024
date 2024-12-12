use std::fs;
use std::collections::HashSet;

struct Plot {
    plant: char,
    in_region: bool,
    fences: usize
}

fn search(row: usize, col: usize, grid: &mut Vec<Vec<Plot>>) -> HashSet<(usize, usize)> {
    let mut region: HashSet<(usize, usize)> = HashSet::new();
    if grid[row][col].in_region {
        return region;
    }
    grid[row][col].in_region = true;
    region.insert((row, col));

    let mut neighbors: Vec<(usize, usize)> = Vec::new();

    if row > 0 {
        neighbors.push((row - 1, col));
    }
    if row < grid.len() - 1 {
        neighbors.push((row + 1, col));
    }
    if col > 0 {
        neighbors.push((row, col - 1));
    }
    if col < grid[0].len() - 1 {
        neighbors.push((row, col + 1));
    }

    let mut fence_sides = 4 - neighbors.len();
    for n in neighbors {
        if grid[n.0][n.1].plant == grid[row][col].plant {
            region.extend(search(n.0, n.1, grid));
        }
        else {
            fence_sides += 1;
        }
    }

    grid[row][col].fences = fence_sides;

    region
}

pub fn main(file: &str) {
    let mut grid: Vec<Vec<Plot>> = fs::read_to_string(file).unwrap().trim().split("\n").map(|l| l.chars().map(|c| Plot{plant: c, in_region: false, fences: 0}).collect()).collect();
    let dims = (grid.len(), grid[0].len());

    let mut regions: Vec<HashSet<(usize, usize)>> = Vec::new();
    for i in 0..dims.0 {
        for j in 0..dims.1 {
            regions.push(search(i, j, &mut grid));
        }
    }

    let mut cost = 0;
    for r in regions {
        let area = r.len();
        let mut perimeter = 0;
        for plot in r {
            perimeter += grid[plot.0][plot.1].fences;
        }
        cost += area * perimeter;
    }

    println!("part 1: {}", cost)
}
