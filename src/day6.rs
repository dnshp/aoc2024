use std::fs;
//use std::error::Error;
use std::collections::HashSet;

struct Area {
    visited: HashSet<(usize, usize)>,
    obs_rows: Vec<Vec<bool>>,
    row: usize,
    col: usize,
    dir: char,
}

impl Area {
    fn propagate(&mut self) -> bool {
        match self.dir {
            '>' => {
                if self.col >= self.obs_rows[0].len() - 1 {
                    return true; // exit the area
                } else if self.obs_rows[self.row][self.col + 1] {
                    self.dir = 'v'; // get rotated, idiot
                } else {
                    self.col += 1;
                }
            },
            'v' => {
                if self.row >= self.obs_rows.len() - 1 {
                    return true; // exit the area
                } else if self.obs_rows[self.row + 1][self.col] {
                    self.dir = '<'; // get rotated, idiot
                } else {
                    self.row += 1;
                }
            },
            '<' => {
                if self.col <= 0 {
                    return true; // exit the area
                } else if self.obs_rows[self.row][self.col - 1] {
                    self.dir = '^'; // get rotated, idiot
                } else {
                    self.col -= 1;
                }
            },
            '^' => {
                if self.row <= 0 {
                    return true; // exit the area
                } else if self.obs_rows[self.row - 1][self.col] {
                    self.dir = '>'; // get rotated, idiot
                } else {
                    self.row -= 1;
                }
            },
            _ => panic!("something wrong!"),
        };

        self.visited.insert((self.row, self.col));
        false
    }
}

fn parse(file: &str) -> Area {
    let rows: Vec<Vec<char>> = fs::read_to_string(file).unwrap()
                                                       .split("\n")
                                                       .filter(|e| e.len() > 0)
                                                       .map(|r| r.chars().collect())
                                                       .collect();

    let mut row = 0;
    let mut col = 0;
    let mut dir: char = '^';

    for r in 0..rows.len() {
        for c in 0..rows[0].len() {
            if rows[r][c] == '>' || rows[r][c] == 'v' || rows[r][c] == '<' || rows[r][c] == '^' {
                row = r;
                col = c;
                dir = rows[r][c];
            }
        }
    }

    let rows: Vec<Vec<bool>> = rows.into_iter().map(|r| r.into_iter().map(|c| c == '#').collect()).collect();
    let mut visited = HashSet::new();
    visited.insert((row, col));

    Area {
        visited : visited,
        obs_rows : rows,
        row : row,
        col : col,
        dir : dir
    }
}

pub fn main(file: &str) {
    let mut area: Area = parse(file);
    while !area.propagate() {}
    println!("{}", area.visited.len());
}
