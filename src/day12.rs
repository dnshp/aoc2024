use std::fs;
use std::collections::HashSet;

struct Plot {
    plant: char,
    in_region: bool,
    fences: usize,
    corners: usize
}

const NORTH: usize = 0;
const SOUTH: usize = 1;
const WEST: usize = 2;
const EAST: usize = 3;
const NORTHWEST: usize = 0;
const NORTHEAST: usize = 1;
const SOUTHWEST: usize = 2;
const SOUTHEAST: usize = 3;

type Pos = (usize, usize);

fn check_corner_geometry(plant: char, neighbor1: Option<&Plot>, neighbor2: Option<&Plot>, corner: Option<&Plot>) -> bool {
    // We claim that the number of sides to a region in part 2 is equal to the number of corners.
    // A corner must satisfy one of these geometries, where X is the plot under consideration, and
    // Y is an adjacent non-matching plot, or an edge of grid (None).
    //
    // ..   .X   X.
    // X.   X.   XX
    //
    // we check for those combinations here.

    let neighbor1_match = neighbor1.is_some() && neighbor1.unwrap().plant == plant;
    let neighbor2_match = neighbor2.is_some() && neighbor2.unwrap().plant == plant;
    let corner_match = corner.is_some() && corner.unwrap().plant == plant;

    // combinations 1 & 2
    if !neighbor1_match && !neighbor2_match {return true;};
    // combination 3
    if neighbor1_match && neighbor2_match && !corner_match {return true;};
    
    false
}

fn search(row: usize, col: usize, grid: &mut Vec<Vec<Plot>>) -> HashSet<Pos> {
    let mut region: HashSet<Pos> = HashSet::new();
    if grid[row][col].in_region {
        return region;
    }
    grid[row][col].in_region = true;
    region.insert((row, col));

    let mut neighbors: [Option<Pos>; 4] = [None, None, None, None];
    let mut corners: [Option<Pos>; 4] = [None, None, None, None];

    // populate neighbors and corners
    // since we do a mutable borrow for part 1 further down, we'll just store the row/col here and
    // resolve the Plot later.
    if row > 0 {
        neighbors[NORTH] = Some((row - 1, col));
    }
    if row < grid.len() - 1 {
        neighbors[SOUTH] = Some((row + 1, col));
    }
    if col > 0 {
        neighbors[WEST] = Some((row, col - 1));
        if neighbors[NORTH].is_some() {
            corners[NORTHWEST] = Some((row - 1, col - 1));
        }
        if neighbors[SOUTH].is_some() {
            corners[SOUTHWEST] = Some((row + 1, col - 1));
        }
    }
    if col < grid[0].len() - 1 {
        neighbors[EAST] = Some((row, col + 1));
        if neighbors[NORTH].is_some() {
            corners[NORTHEAST] = Some((row - 1, col + 1));
        }
        if neighbors[SOUTH].is_some() {
            corners[SOUTHEAST] = Some((row + 1, col + 1));
        }
    }

    // part 1
    let mut fence_sides = 0;
    for n in neighbors {
        match n {
            None => {fence_sides += 1;},
            Some((r, c)) => {
                if grid[r][c].plant == grid[row][col].plant {
                    region.extend(search(r, c, grid));
                }
                else {
                    fence_sides += 1;
                }
            },
        };
    }

    // part 2 -- check corner geometries
    let neighbors: Vec<_> = neighbors.iter().map(|n| match n {
        None => None,
        Some((r, c)) => Some(&grid[*r][*c]),
    }).collect();
    let corners: Vec<_> = corners.iter().map(|c| match c {
        None => None,
        Some((r, c)) => Some(&grid[*r][*c]),
    }).collect();
    let corner_count = vec![
        check_corner_geometry(grid[row][col].plant, neighbors[NORTH], neighbors[WEST], corners[NORTHWEST]),
        check_corner_geometry(grid[row][col].plant, neighbors[NORTH], neighbors[EAST], corners[NORTHEAST]),
        check_corner_geometry(grid[row][col].plant, neighbors[SOUTH], neighbors[WEST], corners[SOUTHWEST]),
        check_corner_geometry(grid[row][col].plant, neighbors[SOUTH], neighbors[EAST], corners[SOUTHEAST])].iter().filter(|c| **c).collect::<Vec<_>>().len();

    grid[row][col].fences = fence_sides;
    grid[row][col].corners = corner_count;

    region
}

pub fn main(file: &str) {
    let mut grid: Vec<Vec<Plot>> = fs::read_to_string(file).unwrap().trim().split("\n").map(|l| l.chars().map(|c| Plot{plant: c, in_region: false, fences: 0, corners: 0}).collect()).collect();
    let dims = (grid.len(), grid[0].len());

    let mut regions: Vec<HashSet<Pos>> = Vec::new();
    for i in 0..dims.0 {
        for j in 0..dims.1 {
            regions.push(search(i, j, &mut grid));
        }
    }

    let mut cost = [0, 0];
    for r in regions {
        let area = r.len();
        let mut perimeter = 0;
        let mut corners = 0;
        for plot in r {
            perimeter += grid[plot.0][plot.1].fences;
            corners += grid[plot.0][plot.1].corners;
        }
        cost[0] += area * perimeter;
        cost[1] += area * corners;
    }

    println!("part 1: {}", cost[0]);
    println!("part 2: {}", cost[1]);
}
