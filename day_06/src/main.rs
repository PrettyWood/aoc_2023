use std::str::FromStr;

use nom::{
    bytes::complete::tag,
    character::complete::{self, multispace1, newline},
    multi::separated_list1,
    IResult,
};

fn main() {
    println!("Part 1: {}", part1(include_str!("input.txt")));
    println!("Part 2: {}", part2(include_str!("input.txt")));
}

#[derive(Debug)]
struct Grid {
    time: Vec<usize>,
    distance: Vec<usize>,
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_grid(s)
            .map(|(_, grid)| grid)
            .map_err(|e| e.to_string())
    }
}

fn parse_grid(input: &str) -> IResult<&str, Grid> {
    let (input, _) = tag("Time:")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, time) = separated_list1(multispace1, complete::u64)(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = tag("Distance:")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, distance) = separated_list1(multispace1, complete::u64)(input)?;
    Ok((
        input,
        Grid {
            time: time.iter().map(|&n| n as usize).collect(),
            distance: distance.iter().map(|&n| n as usize).collect(),
        },
    ))
}

// compute distance after holding for `hold_time` seconds in a race of `race_duration` seconds
fn compute_distance(hold_time: usize, race_duration: usize) -> usize {
    hold_time * (race_duration - hold_time)
}

fn nb_of_ways_to_win(race_duration: usize, record_distance: usize) -> usize {
    (0..=race_duration)
        .filter(|&hold_time| compute_distance(hold_time, race_duration) > record_distance)
        .count()
}

pub fn part1(input: &str) -> usize {
    let grid = Grid::from_str(input).unwrap();
    grid.time
        .iter()
        .zip(grid.distance.iter())
        .map(|(&t, &d)| nb_of_ways_to_win(t, d))
        .product()
}

pub fn part2(input: &str) -> usize {
    let grid = Grid::from_str(input).unwrap();
    let time = grid
        .time
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .join("")
        .parse::<usize>()
        .unwrap();
    let distance = grid
        .distance
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .join("")
        .parse::<usize>()
        .unwrap();
    nb_of_ways_to_win(time, distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = r#"Time:      7  15   30
Distance:  9  40  200"#;
        let grid = Grid::from_str(input).unwrap();
        assert_eq!(grid.time, vec![7, 15, 30]);
        assert_eq!(grid.distance, vec![9, 40, 200]);
    }

    #[test]
    fn test_speed() {
        const RACE_DURATION: usize = 7;
        assert_eq!(compute_distance(0, RACE_DURATION), 0);
        assert_eq!(compute_distance(1, RACE_DURATION), 6);
        assert_eq!(compute_distance(2, RACE_DURATION), 10);
        assert_eq!(compute_distance(3, RACE_DURATION), 12);
        assert_eq!(compute_distance(4, RACE_DURATION), 12);
        assert_eq!(compute_distance(5, RACE_DURATION), 10);
        assert_eq!(compute_distance(6, RACE_DURATION), 6);
        assert_eq!(compute_distance(7, RACE_DURATION), 0);
    }

    #[test]
    fn test_nb_of_ways() {
        assert_eq!(nb_of_ways_to_win(7, 9), 4);
        assert_eq!(nb_of_ways_to_win(15, 40), 8);
        assert_eq!(nb_of_ways_to_win(30, 200), 9);
    }

    #[test]
    fn test_part1() {
        let input = r#"Time:      7  15   30
Distance:  9  40  200"#;
        assert_eq!(part1(input), 288);
    }

    #[test]
    fn test_part2() {
        let input = r#"Time:      7  15   30
Distance:  9  40  200"#;
        assert_eq!(part2(input), 71503);
    }
}
