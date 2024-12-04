use std::fs;
use std::error::Error;

static X: u8 = b'X';
static M: u8 = b'M';
static A: u8 = b'A';
static S: u8 = b'S';

fn find_patterns(lines: &Vec<Vec<u8>>) -> Result<usize, Box<dyn Error>> {
    let mut locs: Vec<(usize, usize)> = Vec::new();
    let mut count = 0;

    for (i, line) in lines.iter().enumerate() {
        for (j, letter) in line.iter().enumerate() {
            if *letter == X {
                locs.push((i, j));
            }
        }
    }

    let cols = lines[0].len();
    let rows = lines.len();

    for loc in locs {
        let row = loc.0;
        let col = loc.1;

        if col < cols - 3 {
            // east row
            if lines[row][col+1] == M && lines[row][col+2] == A && lines[row][col+3] == S {
                count += 1
            }
            // southeast diagonal
            if row < rows - 3 {
                if lines[row+1][col+1] == M && lines[row+2][col+2] == A && lines[row+3][col+3] == S {
                    count += 1
                }
            }
            // northeast diagonal
            if row > 2 {
                if lines[row-1][col+1] == M && lines[row-2][col+2] == A && lines[row-3][col+3] == S {
                    count += 1
                }
            }
        }
        // west row
        if col > 2 {
            if lines[row][col-1] == M && lines[row][col-2] == A && lines[row][col-3] == S {
                count += 1
            }
            // southwest diagonal
            if row < rows - 3 {
                if lines[row+1][col-1] == M && lines[row+2][col-2] == A && lines[row+3][col-3] == S {
                    count += 1
                }
            }
            // northwest diagonal
            if row > 2 {
                if lines[row-1][col-1] == M && lines[row-2][col-2] == A && lines[row-3][col-3] == S {
                    count += 1
                }
            }
        }
        // south column
        if row < rows - 3 {
            if lines[row+1][col] == M && lines[row+2][col] == A && lines[row+3][col] == S {
                count += 1
            }
        }
        // north column
        if row > 2 {
            if lines[row-1][col] == M && lines[row-2][col] == A && lines[row-3][col] == S {
                count += 1
            }
        }
    }

    Ok(count)
}

fn find_patterns2(lines: &Vec<Vec<u8>>) -> Result<usize, Box<dyn Error>> {
    let mut locs: Vec<(usize, usize)> = Vec::new();
    let mut count = 0;

    for (i, line) in lines.iter().enumerate() {
        for (j, letter) in line.iter().enumerate() {
            if *letter == A {
                locs.push((i, j));
            }
        }
    }

    let cols = lines[0].len();
    let rows = lines.len();

    for loc in locs {
        let row = loc.0;
        let col = loc.1;

        if col == 0 || col == cols - 1 || row == 0 || row == rows - 1 {
            continue;
        }

        // populate clockwise
        let corners = vec![lines[row-1][col-1], lines[row-1][col+1], lines[row+1][col+1], lines[row+1][col-1]];
        // now as long as opposite corners aren't the same, and we have 2 Ms and 2 Ss, we're good!
        if corners.iter().filter(|n| **n == M).count() == 2 && corners.iter().filter(|n| **n == S).count() == 2 && corners[0] != corners[2] {
            count += 1
        }
    }

    Ok(count)
}

fn main() {
    let filename = "dat/input.txt";

    let text: String = fs::read_to_string(filename).unwrap();
    let lines: Vec<Vec<u8>> = text.split("\n").filter(|e| e.len() > 0).map(|v| Vec::from(v)).collect();
    for line in &lines {
        assert_eq!(lines[0].len(), line.len());
    }

    let count = find_patterns(&lines).unwrap();

    println!("{}", count);

    let count2 = find_patterns2(&lines).unwrap();

    println!("{}", count2);
}
