mod days;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Which day to run
    #[arg(short, long)]
    day: u8,

    // Which part to run
    #[arg(short, long)]
    part: u8,
}

fn main() {
    let args = Args::parse();
    let day = match args.day {
        1..=25 => format!("{:02}", args.day),
        _ => panic!("Invalid day"),
    };
    let part = match args.part {
        1 | 2 => args.part,
        _ => panic!("Invalid part"),
    };
    let input = std::fs::read_to_string(format!("src/inputs/day_{}/part{}.txt", day, part))
        .expect("Failed to read input file");
    let result = match (day.as_str(), part) {
        ("01", 1) => days::day_01::part1(&input),
        ("01", 2) => days::day_01::part2(&input),
        _ => todo!("Not implemented yet"),
    };
    println!("Day {}, part {}: {}", day, part, result);
}