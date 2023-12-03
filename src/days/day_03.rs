#[derive(Debug)]
struct Number {
    value: usize,
    y: usize,
    x1: usize,
    x2: usize,
}

impl Number {
    fn from_digits(digits: Vec<(usize, char)>, line_idx: usize) -> Self {
        let number = digits.iter().map(|(_, c)| c).collect::<String>();
        Number {
            value: number.parse::<usize>().unwrap(),
            y: line_idx,
            x1: digits.first().unwrap().0,
            x2: digits.last().unwrap().0,
        }
    }

    fn is_close_to_symbol(&self, symbol: &Symbol) -> bool {
        let left = if self.x1 == 0 { 0 } else { self.x1 - 1 };
        let right = self.x2 + 1;
        (self.y).abs_diff(symbol.y) <= 1 && (left..=right).contains(&symbol.x)
    }
}

#[derive(Debug)]
struct Symbol {
    value: char,
    y: usize,
    x: usize,
}

#[derive(Debug)]
struct Grid {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

fn parse_line(line: &str, line_idx: usize) -> (Vec<Number>, Vec<Symbol>) {
    let mut numbers: Vec<Number> = vec![];
    let mut symbols: Vec<Symbol> = vec![];
    let mut digits: Vec<(usize, char)> = vec![];
    for (i, c) in line.char_indices() {
        match c {
            '.' => (),
            '0'..='9' => {
                digits.push((i, c));
                continue;
            }
            c => symbols.push(Symbol {
                value: c,
                y: line_idx,
                x: i,
            }),
        }
        if !digits.is_empty() {
            numbers.push(Number::from_digits(digits, line_idx));
            digits = vec![];
        }
    }
    if !digits.is_empty() {
        numbers.push(Number::from_digits(digits, line_idx));
    }
    (numbers, symbols)
}

pub fn part1(input: &str) -> usize {
    let grid = input
        .lines()
        .enumerate()
        .map(|(i, line)| parse_line(line, i))
        .fold(
            Grid {
                numbers: vec![],
                symbols: vec![],
            },
            |mut grid, (numbers, symbols)| {
                grid.numbers.extend(numbers);
                grid.symbols.extend(symbols);
                grid
            },
        );
    grid.numbers
        .iter()
        .filter(|n| grid.symbols.iter().any(|s| n.is_close_to_symbol(s)))
        .map(|n| n.value)
        .sum()
}

pub fn part2(input: &str) -> usize {
    let grid = input
        .lines()
        .enumerate()
        .map(|(i, line)| parse_line(line, i))
        .fold(
            Grid {
                numbers: vec![],
                symbols: vec![],
            },
            |mut grid, (numbers, symbols)| {
                grid.numbers.extend(numbers);
                grid.symbols.extend(symbols);
                grid
            },
        );
    grid.symbols
        .iter()
        .filter(|s| s.value == '*')
        .map(|s| {
            let close_numbers = grid
                .numbers
                .iter()
                .filter(|n| n.is_close_to_symbol(s))
                .collect::<Vec<_>>();
            if close_numbers.len() == 2 {
                close_numbers[0].value * close_numbers[1].value
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        assert_eq!(part1(input), 4361);
    }

    #[test]
    fn test_part2() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        assert_eq!(part2(input), 467835);
    }
}
