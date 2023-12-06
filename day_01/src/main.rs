fn main() {
    println!("Part 1: {}", part1(include_str!("input.txt")));
    println!("Part 2: {}", part2(include_str!("input.txt")));
}

pub fn part1(input: &str) -> usize {
    input.lines().map(to_two_digits).sum()
}

fn to_two_digits(line: &str) -> usize {
    let first_digit = line.find(|c: char| c.is_ascii_digit()).unwrap();
    let last_digit = line.rfind(|c: char| c.is_ascii_digit()).unwrap();
    format!(
        "{}{}",
        &line.chars().nth(first_digit).unwrap(),
        &line.chars().nth(last_digit).unwrap()
    )
    .parse::<usize>()
    .unwrap()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(letters_to_digits)
        .map(|line| to_two_digits(&line))
        .sum()
}

const LETTERS_TO_DIGITS: [(&str, &str); 9] = [
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
];

fn letters_to_digits(line: &str) -> String {
    change_last_letters_to_digits(&change_first_letters_to_digits(line))
}

fn change_first_letters_to_digits(line: &str) -> String {
    let first_char = line.chars().next().unwrap();
    if first_char.is_ascii_digit() {
        return line.to_string();
    }

    for (word, digit) in LETTERS_TO_DIGITS.iter() {
        if line.starts_with(word) {
            return format!("{}{}", digit, &line.strip_prefix(word).unwrap());
        }
    }
    format!(
        "{}{}",
        &first_char,
        change_first_letters_to_digits(&line[1..])
    )
}

fn change_last_letters_to_digits(line: &str) -> String {
    let last_char = line.chars().last().unwrap();
    if last_char.is_ascii_digit() {
        return line.to_string();
    }

    for (word, digit) in LETTERS_TO_DIGITS.iter() {
        if line.ends_with(word) {
            return format!("{}{}", &line.strip_suffix(word).unwrap(), digit);
        }
    }
    format!(
        "{}{}",
        change_last_letters_to_digits(&line[..line.len() - 1]),
        last_char
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        assert_eq!(part1(input), 142);
    }

    #[test]
    fn test_letters_to_digits() {
        assert_eq!(letters_to_digits("two1nine"), "219");
        assert_eq!(letters_to_digits("eightwothree"), "8wo3");
        assert_eq!(letters_to_digits("xtwone3four"), "x2ne34");
        assert_eq!(letters_to_digits("zoneight234"), "z1ight234");
    }

    #[test]
    fn test_part2() {
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
        assert_eq!(part2(input), 281);
    }
}
