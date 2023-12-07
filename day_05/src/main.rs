use std::{collections::HashSet, str::FromStr};

use nom::{
    self,
    bytes::complete::tag,
    character::complete::{self, alpha1, newline, space1},
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair, tuple},
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    println!("Part 1: {}", part1(include_str!("input.txt")));
    println!("Part 2: {}", part2(include_str!("input.txt")));
}

#[derive(Debug)]
struct TMap(Vec<(usize, usize, usize)>);

fn parse_tmap(input: &str) -> nom::IResult<&str, TMap> {
    let (input, (_source, _destination)) = separated_pair(alpha1, tag("-to-"), alpha1)(input)?;
    let (input, _) = tag(" map:")(input)?;
    let (input, _) = newline(input)?;
    let (input, ranges) = separated_list1(
        newline,
        tuple((complete::u64, space1, complete::u64, space1, complete::u64)),
    )(input)?;
    let ranges = ranges
        .iter()
        .map(|&(d, _, s, _, r)| (s as usize, d as usize, r as usize))
        .collect::<Vec<(usize, usize, usize)>>();
    Ok((input, TMap(ranges)))
}

impl TMap {
    fn convert(&self, n: usize) -> usize {
        self.0
            .iter()
            .find(|&&(s, _, r)| (s..s + r).contains(&n))
            .map(|&(s, d, _)| d + (n - s))
            .unwrap_or(n)
    }
}

#[derive(Debug)]
struct TGrid {
    seeds: Vec<usize>,
    maps: Vec<TMap>,
}

fn parse_grid(input: &str) -> nom::IResult<&str, TGrid> {
    let (input, seeds) = preceded(tag("seeds: "), separated_list1(space1, complete::u64))(input)?;
    let (input, maps) = many1(preceded(many1(newline), parse_tmap))(input)?;
    Ok((
        input,
        TGrid {
            seeds: seeds.iter().map(|s| *s as usize).collect::<Vec<_>>(),
            maps,
        },
    ))
}

impl FromStr for TGrid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_grid(s)
            .map(|(_, grid)| grid)
            .map_err(|e| e.to_string())
    }
}

impl TGrid {
    fn find_location(&self, seed: usize) -> usize {
        self.maps.iter().fold(seed, |seed, map| map.convert(seed))
    }
}

pub fn part1(input: &str) -> usize {
    let grid = TGrid::from_str(input).unwrap();
    grid.seeds
        .iter()
        .map(|seed| grid.find_location(*seed))
        .min()
        .unwrap()
}

pub fn part2(input: &str) -> usize {
    let grid = TGrid::from_str(input).unwrap();
    let seeds_ranges = grid
        .seeds
        .chunks(2)
        .map(|c| c[0]..c[0] + c[1])
        .collect::<HashSet<_>>();
    seeds_ranges
        .par_iter()
        .flat_map(|range| range.clone())
        .map(|seed| grid.find_location(seed))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;
        assert_eq!(part1(input), 35);
    }

    #[test]
    fn test_part2() {
        let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;
        assert_eq!(part2(input), 46);
    }
}
