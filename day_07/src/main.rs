mod part1;
mod part2;

use part1::part1;
use part2::part2;

fn main() {
    println!("Part 1: {}", part1(include_str!("input.txt")));
    println!("Part 2: {}", part2(include_str!("input.txt")));
}
