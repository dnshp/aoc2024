use std::fs;
use std::collections::HashSet;

fn search(row: usize, col: usize, grid: &Vec<Vec<char>>, in_region: &mut Vec<Vec<bool>>, fences: &mut Vec<Vec<usize>>) -> HashSet<(usize, usize)> {
    let mut region: HashSet<(usize, usize)> = HashSet::new();
    if in_region[row][col] {
        return region;
    }
    in_region[row][col] = true;
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
        if grid[n.0][n.1] == grid[row][col] {
            region.extend(search(n.0, n.1, grid, in_region, fences));
        }
        else {
            fence_sides += 1;
        }
    }

    fences[row][col] = fence_sides;

    region
}

pub fn main(file: &str) {
    let grid: Vec<Vec<char>> = fs::read_to_string(file).unwrap().trim().split("\n").map(|l| l.chars().collect()).collect();
    let dims = (grid.len(), grid[0].len());

    let mut in_region: Vec<Vec<bool>> = Vec::new(); // already included in a region
    let mut fences: Vec<Vec<usize>> = Vec::new(); // how many fences on this plot

    for _i in 0..dims.0 {
        let mut region_row: Vec<bool> = Vec::new();
        let mut fence_row: Vec<usize> = Vec::new();
        for _j in 0..dims.1 {
            region_row.push(false);
            fence_row.push(0);
        }
        in_region.push(region_row);
        fences.push(fence_row);
    }

    let mut regions: Vec<HashSet<(usize, usize)>> = Vec::new();
    for i in 0..dims.0 {
        for j in 0..dims.1 {
            regions.push(search(i, j, &grid, &mut in_region, &mut fences));
        }
    }

    let mut cost = 0;
    for r in regions {
        let area = r.len();
        let mut perimeter = 0;
        for plot in r {
            perimeter += fences[plot.0][plot.1];
        }
        cost += area * perimeter;
    }

    println!("part 1: {}", cost)
}
