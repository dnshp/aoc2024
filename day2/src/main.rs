use std::fs;
use std::error::Error;

fn parse_reports(f: &str) -> Result<Vec<Vec<i64>>, Box<dyn Error>> {
    let lines: String = fs::read_to_string(f)?;
    let lines = lines.split("\n").filter(|e| e.len() > 0);

    let mut reports: Vec<Vec<i64>> = Vec::new();
    for line in lines {
        let report: Vec<i64> = line.split(" ").filter(|e| e.len() > 0).map(|x| x.parse::<i64>().unwrap()).collect();
        reports.push(report);
    }
    Ok(reports)
}

fn evaluate_report(report: &Vec<i64>) -> Result<u64, Box<dyn Error>> {
    if report.len() < 2 {
        return Ok(1);
    }

    let ascending = report[1] > report[0];
    for i in 1..report.len() {
        let diff = report[i] - report[i-1];
        if ascending && (diff > 3 || diff < 1) {
            return Ok(0);
        } else if !ascending && (diff < -3 || diff > -1) {
            return Ok(0);
        }
    }
    Ok(1)
}

fn main() {
    let reports = parse_reports("dat/input.txt").unwrap();
    // let safe: Vec<u64> = reports.into_iter().map(|x| evaluate_report(&x).unwrap()).collect();
    // for i in 35..40 {
    //     println!("{} {}", i, safe[i]);
    // }
    let safe: u64 = reports.into_iter().map(|x| evaluate_report(&x).unwrap()).sum();
    println!("{} safe reports.", safe);
}
