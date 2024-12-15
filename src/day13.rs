use std::fs;
use std::error::Error;
use regex::Regex;

#[derive(Debug)]
struct Machine {
    a: (f64, f64),
    b: (f64, f64),
    p: (f64, f64)
}

fn parse_machine(lines: Vec<&str>) -> Result<Machine, Box<dyn Error>> {
    assert_eq!(lines.len(), 3);
    let res = vec![
        r"Button A: X\+(\d+), Y\+(\d+)",
        r"Button B: X\+(\d+), Y\+(\d+)",
        r"Prize: X=(\d+), Y=(\d+)"
    ];
    
    let a = Regex::new(res[0]).unwrap().captures(lines[0]).unwrap();
    let a: (f64, f64) = (a[1].parse()?, a[2].parse()?);

    let b = Regex::new(res[1]).unwrap().captures(lines[1]).unwrap();
    let b: (f64, f64) = (b[1].parse()?, b[2].parse()?);

    let p = Regex::new(res[2]).unwrap().captures(lines[2]).unwrap();
    let p: (f64, f64) = (p[1].parse()?, p[2].parse()?);

    Ok(Machine{a: a, b: b, p: p})
}

pub fn main(file: &str) {
    let machines = fs::read_to_string(file).expect("File not found");
    let machines: Vec<_> = machines.split("\n\n").map(|m| parse_machine(m.trim().split("\n").collect::<Vec<_>>()).unwrap()).collect();

    // linear system of equations satisfying
    // AX_a + BX_b = P_x
    // AY_a + BY_b = P_y
    //
    // ie.
    //
    // | X_a X_b | |A| = |P_x|
    // | Y_a Y_b | |B|   |P_y|
    //
    // So inverting the matrix gives us A and B.

    let mut total = 0;
    for m in machines {
        let scale = 1.0 / ((m.a.0 * m.b.1) - (m.a.1 * m.b.0));
        let abcd_inv = vec![m.b.1 * scale, -1.0 * m.b.0 * scale, -1.0 * m.a.1 * scale, m.a.0 * scale];
        let presses = vec![
            (abcd_inv[0] * m.p.0 + abcd_inv[1] * m.p.1).round() as usize,
            (abcd_inv[2] * m.p.0 + abcd_inv[3] * m.p.1).round() as usize
        ];
        // if no solution, discard and continue
        if (m.a.0 as usize * presses[0]) + (m.b.0 as usize * presses[1]) != m.p.0 as usize {
            continue;
        }
        if (m.a.1 as usize * presses[0]) + (m.b.1 as usize * presses[1]) != m.p.1 as usize {
            continue;
        }
        if presses[0] <= 100 && presses[1] <= 100 {
            total += (3 * presses[0]) + presses[1];
        }
    }

    println!("{}", total);
}
