use std::fs;

pub fn main(file: &str) {
    let lines: Vec<Vec<u64>> = fs::read_to_string(file).unwrap().split("\n").filter(|e| e.len() > 0).map(|l| l.split(|c| c == ':' || c == ' ').filter(|n| n.len() > 0).map(|n| n.parse().unwrap()).collect()).collect();
    let mut total: u64 = 0;
    for line in lines {
        let operators = line.len() - 2;
        assert!(operators > 0);
        for c in 0..(1 << operators) {
            let mut rolling_val: u64 = line[1];
            for (i, operand) in line[2..].iter().enumerate() {
                let mask = 1 << i;
                if mask & c == mask {
                    rolling_val *= operand;
                } else {
                    rolling_val += operand;
                }
            }
            if rolling_val == line[0] {
                total += rolling_val;
                break;
            }
        }
    }
    println!("{}", total);
}
