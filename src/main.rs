pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 3);
    let day = &args[1].trim()[..];
    let file = &args[2].trim()[..];

    match day {
        "1" => day1::main(file),
        "2" => day2::main(file),
        "3" => day3::main(file),
        "4" => day4::main(file),
        "5" => day5::main(file),
        "6" => day6::main(file),
        _ => panic!("invalid day {}", day),
    }
}
