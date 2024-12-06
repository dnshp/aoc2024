use std::fs;
use std::error::Error;
use std::cmp::Ordering;

type Rule = (u32, u32);
type Sequence = Vec<u32>;

fn parse(filename: &str) -> Result<(Vec<Rule>, Vec<Sequence>), Box<dyn Error>> {
    let text: String = fs::read_to_string(filename).unwrap();
    let text: Vec<&str> = text.split("\n").filter(|e| e.len() > 0).collect();

    let mut rules: Vec<Rule> = Vec::new();
    let mut sequences: Vec<Sequence> = Vec::new();

    for line in text {
        if line.find("|").is_some() {
            assert!(line.find(",").is_none());
            let rule: Vec<&str> = line.split("|").collect();
            assert_eq!(rule.len(), 2);
            let rule: Rule = (rule[0].parse()?, rule[1].parse()?);
            rules.push(rule);
        } else if line.find(",").is_some() {
            let mut sequence: Sequence = Vec::new();
            for page in line.split(",") {
                sequence.push(page.parse()?);
            }
            sequences.push(sequence);
        }
    }

    Ok((rules, sequences))
}

// fn check_rule_for_sequence(r: &Rule, s: &Sequence) -> bool {
//     let first = s.iter().position(|x| *x == r.0);
//     let second = s.iter().position(|x| *x == r.1);
//     if first.is_none() || second.is_none() {
//         return true;
//     }
//     first.unwrap() < second.unwrap()
// }

fn is_sorted_by_rules(a: &u32, b: &u32, rules: &Vec<Rule>) -> bool {
    for r in rules {
        if r.0 == *a && r.1 == *b {
            return true;
        } else if r.0 == *b && r.1 == *a {
            return false;
        }
    }
    panic!("Values {} and {} did not have a corresponding sort rule!", a, b);
}

fn compare_by_rules(a: &u32, b: &u32, rules: &Vec<Rule>) -> Ordering {
    match is_sorted_by_rules(a, b, rules) {
        true => Ordering::Less,
        false => Ordering::Greater
    }
}

pub fn main(file: &str) {
    let (rules, sequences) = parse(file).unwrap();

    let mut answers = vec![0, 0];
    for mut sequence in sequences {
        let order_correct: bool = sequence.is_sorted_by(|a, b| is_sorted_by_rules(a, b, &rules));
        if order_correct {
            assert_eq!(sequence.len() % 2, 1);
            // inc part 1 answer
            answers[0] += sequence[(sequence.len() - 1) / 2];
        } else {
            sequence.sort_by(|a, b| compare_by_rules(a, b, &rules));
            answers[1] += sequence[(sequence.len() - 1) / 2];
        }
    }
    println!("Part 1: {}", answers[0]);
    println!("Part 2: {}", answers[1]);
}
