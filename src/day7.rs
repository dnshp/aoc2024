use std::fs;
use std::error::Error;
use itertools::Itertools;

fn test_operator_combinations(expected_total: u64, operands: &[u64], operators: &[&str]) -> Result<bool, Box<dyn Error>> {
    let num_operators = operands.len() - 1;
    let operator_combinations = (0..num_operators).map(|_| operators).multi_cartesian_product();

    for combo in operator_combinations {
        let mut rolling_val = operands[0];
        for (i, operand) in operands[1..].iter().enumerate() {
            match *combo[i] {
                "*" => { rolling_val *= operand; },
                "+" => { rolling_val += operand; },
                "||" => { rolling_val = (rolling_val.to_string() + &operand.to_string()).parse()?; },
                op => panic!("illegal operator {}", op)
            };
        }
        if rolling_val == expected_total {
            return Ok(true);
        }
    }
    return Ok(false);
}

pub fn main(file: &str) {
    let lines: Vec<Vec<u64>> = fs::read_to_string(file).unwrap().split("\n").filter(|e| e.len() > 0).map(|l| l.split(|c| c == ':' || c == ' ').filter(|n| n.len() > 0).map(|n| n.parse().unwrap()).collect()).collect();
    let mut totals = vec![0, 0];
    for line in lines {
        if test_operator_combinations(line[0], &line[1..], &["+", "*"]).unwrap() {
            totals[0] += line[0];
        }
        if test_operator_combinations(line[0], &line[1..], &["+", "*", "||"]).unwrap() {
            totals[1] += line[0];
        }
    }
    println!("{:?}", totals);
}
