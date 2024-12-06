use std::fs;
//use std::error::Error;
use std::collections::HashSet;

struct Area {
    visited: HashSet<(usize, usize)>,
    obs_rows: Vec<Vec<bool>>,
    row: usize,
    col: usize,
    dir: char,
    visited_with_dir: HashSet<(usize, usize, char)>,
}

struct InfiniteLoop;

impl Area {
    fn propagate(&mut self) -> Result<bool, InfiniteLoop> {
        match self.dir {
            '>' => {
                if self.col >= self.obs_rows[0].len() - 1 {
                    return Ok(true); // exit the area
                } else if self.obs_rows[self.row][self.col + 1] {
                    self.dir = 'v'; // get rotated, idiot
                } else {
                    self.col += 1;
                }
            },
            'v' => {
                if self.row >= self.obs_rows.len() - 1 {
                    return Ok(true); // exit the area
                } else if self.obs_rows[self.row + 1][self.col] {
                    self.dir = '<'; // get rotated, idiot
                } else {
                    self.row += 1;
                }
            },
            '<' => {
                if self.col <= 0 {
                    return Ok(true); // exit the area
                } else if self.obs_rows[self.row][self.col - 1] {
                    self.dir = '^'; // get rotated, idiot
                } else {
                    self.col -= 1;
                }
            },
            '^' => {
                if self.row <= 0 {
                    return Ok(true); // exit the area
                } else if self.obs_rows[self.row - 1][self.col] {
                    self.dir = '>'; // get rotated, idiot
                } else {
                    self.row -= 1;
                }
            },
            _ => panic!("something wrong!"),
        };

        if self.visited_with_dir.contains(&(self.row, self.col, self.dir)) {
            return Err(InfiniteLoop);
        }
        self.visited.insert((self.row, self.col));
        self.visited_with_dir.insert((self.row, self.col, self.dir));
        Ok(false)
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
    let mut visited_with_dir = HashSet::new();
    visited_with_dir.insert((row, col, dir));

    Area {
        visited : visited,
        obs_rows : rows,
        row : row,
        col : col,
        dir : dir,
        visited_with_dir : visited_with_dir,
    }
}

pub fn main(file: &str) {
    let mut area: Area = parse(file);
    loop {
        match area.propagate() {
            Ok(true) => break,
            Ok(false) => {},
            Err(_) => panic!("unexpected infinite loop!"),
        }
    }
    println!("{}", area.visited.len());

    let visited = area.visited.clone();
    let mut infinite_solutions = 0;
    for candidate in visited {
        let mut area: Area = parse(file);
        assert!(!area.obs_rows[candidate.0][candidate.1]);
        area.obs_rows[candidate.0][candidate.1] = true;
        loop {
            match area.propagate() {
                Ok(true) => {
                    break;
                },
                Ok(false) => {},
                Err(InfiniteLoop) => {
                    infinite_solutions += 1;
                    break;
                },
            }
        }
    }
    println!("{}", infinite_solutions);
}
