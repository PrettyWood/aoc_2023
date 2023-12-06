use std::{collections::HashMap, str::FromStr};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space0, space1},
    combinator::map_res,
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

fn main() {
    println!("Part 1: {}", part1(include_str!("input.txt")));
    println!("Part 2: {}", part2(include_str!("input.txt")));
}

#[derive(Debug)]
struct Card {
    id: usize,
    winning_numbers: Vec<usize>,
    my_numbers: Vec<usize>,
}

fn list_numbers_parser(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, _) = space0(input)?;
    separated_list0(space1, map_res(digit1, str::parse))(input)
}

fn card_parser(input: &str) -> IResult<&str, Card> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = space1(input)?;
    let (input, id) = nom::character::complete::digit1(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, (left, right)) =
        separated_pair(list_numbers_parser, tag(" | "), list_numbers_parser)(input)?;
    Ok((
        input,
        Card {
            id: id.parse().unwrap(),
            winning_numbers: left,
            my_numbers: right,
        },
    ))
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        card_parser(s)
            .map(|(_, card)| card)
            .map_err(|e| e.to_string())
    }
}

impl Card {
    fn matching_numbers(&self) -> Vec<usize> {
        self.my_numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .copied()
            .collect()
    }

    fn points(&self) -> usize {
        if self.matching_numbers().is_empty() {
            0
        } else {
            2_usize.pow(self.matching_numbers().len() as u32 - 1)
        }
    }
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| Card::from_str(line).unwrap().points())
        .sum()
}

pub fn part2(input: &str) -> usize {
    let cards = input
        .lines()
        .map(|line| Card::from_str(line).unwrap())
        .collect::<Vec<_>>();
    cards
        .iter()
        .fold(HashMap::new(), |mut occurences, card| {
            occurences.entry(card.id).or_insert(1);
            for number in 1..=card.matching_numbers().len() {
                *occurences.entry(card.id + number).or_insert(1) +=
                    *occurences.get(&card.id).unwrap();
            }
            occurences
        })
        .values()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
        assert_eq!(part1(input), 13);
    }

    #[test]
    fn test_part2() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
        assert_eq!(part2(input), 30);
    }
}
