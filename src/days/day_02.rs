use regex::Regex;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Game {
    id_number: u8,
    sets: Vec<Set>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameError;

impl FromStr for Game {
    type Err = ParseGameError;

    // "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
    // -> Game { id_number: 1, sets: [Set {...}, Set {...}, Set {...}] }
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"Game (?<id_number>\d+): (?<sets>.*)").unwrap();
        let caps = re.captures(s).unwrap();
        let sets = caps["sets"]
            .to_string()
            .split("; ")
            .map(|s| s.parse::<Set>().unwrap())
            .collect();
        Ok(Game {
            id_number: caps["id_number"].parse().unwrap(),
            sets,
        })
    }
}

impl Game {
    fn is_possible(&self, elf_set: &Set) -> bool {
        self.sets.iter().all(|set| set.is_possible(elf_set))
    }

    // fewest number of cubes of each color that could have been in the bag to make the game possible
    fn minimal_viable_set(&self) -> Set {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for set in &self.sets {
            if set.red > red {
                red = set.red;
            }
            if set.green > green {
                green = set.green;
            }
            if set.blue > blue {
                blue = set.blue;
            }
        }
        Set { red, green, blue }
    }
}

#[derive(Debug, PartialEq)]
struct Set {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseSetError;

impl FromStr for Set {
    type Err = ParseSetError;

    // "1 red, 2 green, 6 blue" -> Set { red: 1, green: 2, blue: 6 }
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for sub_s in s.split(", ") {
            let re = Regex::new(r"(?<number>\d+) (?<color>\S+)").unwrap();
            let caps = re.captures(sub_s).unwrap();
            let number = caps["number"].parse().unwrap();
            let color = caps["color"].to_string();
            match color.as_str() {
                "red" => red = number,
                "green" => green = number,
                "blue" => blue = number,
                c => panic!("Invalid color: {}", c),
            }
        }
        Ok(Set { red, green, blue })
    }
}

impl Set {
    fn is_possible(&self, elf_set: &Set) -> bool {
        self.red <= elf_set.red && self.green <= elf_set.green && self.blue <= elf_set.blue
    }

    fn power(&self) -> u32 {
        self.red as u32 * self.green as u32 * self.blue as u32
    }
}

pub fn part1(input: &str) -> u32 {
    let games: Vec<Game> = input.lines().map(|s| s.parse().unwrap()).collect();
    let elf_set = Set {
        red: 12,
        green: 13,
        blue: 14,
    };
    games
        .iter()
        .filter(|game| game.is_possible(&elf_set))
        .map(|game| game.id_number as u32)
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let games: Vec<Game> = input.lines().map(|s| s.parse().unwrap()).collect();
    games
        .iter()
        .map(|game| game.minimal_viable_set())
        .map(|set| set.power())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game: Game = input.parse().unwrap();
        assert_eq!(
            game,
            Game {
                id_number: 1,
                sets: vec![
                    Set {
                        red: 4,
                        green: 0,
                        blue: 3,
                    },
                    Set {
                        red: 1,
                        green: 2,
                        blue: 6,
                    },
                    Set {
                        red: 0,
                        green: 2,
                        blue: 0,
                    },
                ]
            }
        );
    }

    #[test]
    fn test_part1() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        assert_eq!(part1(input), 8);
    }

    #[test]
    fn test_part2() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        assert_eq!(part2(input), 2286);
    }
}
