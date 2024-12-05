use std::fs;
use std::error::Error;

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

// fn sort_by_rules(a: u32, b: u32, &rules: Vec<Rule>) {
//     for rule in rules {
//         if rule.0 == a && rule.1 == b {
// 
//         }
//     }
// }

fn check_rule_for_sequence(r: &Rule, s: &Sequence) -> bool {
    let first = s.iter().position(|x| *x == r.0);
    let second = s.iter().position(|x| *x == r.1);
    if first.is_none() || second.is_none() {
        return true;
    }
    first.unwrap() < second.unwrap()
}

fn main() {
    let (rules, sequences) = parse("dat/input.txt").unwrap();

    // // Push the first rule to the ordering, then repeatedly loop through rules to find one that
    // // contains an already seen page number. Poor man's insertion sort.
    // let order: Vec<String> = Vec::new();
    // let first_rule: Rule = rules.remove(0);
    // order.push(first_rule.0);
    // order.push(first_rule.1);

    // for rule in rules {
    //     if order.contains(rule.0) {
    //         if !order.contains(rule.1)
    // }
    
    // part 1
    let mut total = 0;
    for sequence in sequences {
        let order_correct: bool = rules.iter().map(|r| check_rule_for_sequence(r, &sequence)).all(|b| b);
        if order_correct {
            assert_eq!(sequence.len() % 2, 1);
            total += sequence[(sequence.len() - 1) / 2];
        }
    }
    println!("{}", total);
}
