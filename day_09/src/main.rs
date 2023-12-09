use std::collections::VecDeque;

fn main() {
    println!("Part 1: {}", part1(include_str!("input.txt")));
    println!("Part 2: {}", part2(include_str!("input.txt")));
}

/// [10, 13, 16, 21, 30, 45] -> [3, 3, 3, 3, 3]
fn difference_of_line(numbers: &[isize]) -> Vec<isize> {
    numbers.windows(2).map(|w| w[1] - w[0]).collect()
}

fn all_differences(numbers: &[isize]) -> Vec<Vec<isize>> {
    let mut res = vec![numbers.to_vec()];
    while res.last().unwrap().iter().any(|&n| n != 0) {
        res.push(difference_of_line(res.last().unwrap()));
    }
    res
}

fn new_history_right(history: &[Vec<isize>]) -> Vec<Vec<isize>> {
    let mut res = history.to_vec();
    debug_assert!(res.last().unwrap().iter().all(|&n| n == 0));
    let mut previous = 0;
    for line in res.iter_mut().rev() {
        line.push(line.last().unwrap() + previous);
        previous = *line.last().unwrap();
    }
    res
}

fn compute_history_right(numbers: &[isize]) -> Vec<Vec<isize>> {
    let history = all_differences(numbers);
    new_history_right(&history)
}

fn parse_input(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect()
}

pub fn part1(input: &str) -> isize {
    let numbers = parse_input(input);
    numbers
        .iter()
        .map(|numbers| {
            *compute_history_right(numbers)
                .first()
                .unwrap()
                .last()
                .unwrap()
        })
        .sum()
}

fn new_history_left(history: &[VecDeque<isize>]) -> Vec<VecDeque<isize>> {
    let mut res = history.to_vec();
    debug_assert!(res.last().unwrap().iter().all(|&n| n == 0));
    let mut previous = 0;
    for line in res.iter_mut().rev() {
        line.push_front(line.front().unwrap() - previous);
        previous = *line.front().unwrap();
    }
    res
}

fn compute_history_left(numbers: &[isize]) -> Vec<VecDeque<isize>> {
    let history = all_differences(numbers);
    let history: Vec<VecDeque<isize>> = history
        .iter()
        .map(|line| VecDeque::from(line.to_vec()))
        .collect();
    new_history_left(&history)
}

#[allow(unused_variables)]
pub fn part2(input: &str) -> isize {
    let numbers = parse_input(input);
    numbers
        .iter()
        .map(|numbers| {
            *compute_history_left(numbers)
                .first()
                .unwrap()
                .front()
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_difference_of_line() {
        assert_eq!(difference_of_line(&[0, 3, 6, 9, 12, 15]), &[3, 3, 3, 3, 3]);
        assert_eq!(difference_of_line(&[3, 3, 3, 3, 3]), &[0, 0, 0, 0]);
    }

    #[test]
    fn test_all_differences() {
        let input = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(
            all_differences(&input),
            vec![
                vec![0, 3, 6, 9, 12, 15],
                vec![3, 3, 3, 3, 3],
                vec![0, 0, 0, 0],
            ]
        );
    }

    #[test]
    fn test_new_history_right() {
        let input = vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![3, 3, 3, 3, 3],
            vec![0, 0, 0, 0],
        ];
        assert_eq!(
            new_history_right(&input),
            vec![
                vec![0, 3, 6, 9, 12, 15, 18],
                vec![3, 3, 3, 3, 3, 3],
                vec![0, 0, 0, 0, 0],
            ]
        );
    }

    #[test]
    fn test_compute_history_right() {
        let input = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(
            compute_history_right(&input),
            vec![
                vec![0, 3, 6, 9, 12, 15, 18],
                vec![3, 3, 3, 3, 3, 3],
                vec![0, 0, 0, 0, 0],
            ]
        );
    }

    #[test]
    fn test_compute_history_right_2() {
        let input = vec![1, 3, 6, 10, 15, 21];
        assert_eq!(
            compute_history_right(&input),
            vec![
                vec![1, 3, 6, 10, 15, 21, 28],
                vec![2, 3, 4, 5, 6, 7],
                vec![1, 1, 1, 1, 1],
                vec![0, 0, 0, 0],
            ]
        );
    }

    #[test]
    fn test_part1() {
        let input = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;
        assert_eq!(part1(input), 114);
    }

    #[test]
    fn test_new_history_left() {
        let input = vec![
            VecDeque::from(vec![0, 3, 6, 9, 12, 15]),
            VecDeque::from(vec![3, 3, 3, 3, 3]),
            VecDeque::from(vec![0, 0, 0, 0]),
        ];
        assert_eq!(
            new_history_left(&input),
            vec![
                VecDeque::from(vec![-3, 0, 3, 6, 9, 12, 15]),
                VecDeque::from(vec![3, 3, 3, 3, 3, 3]),
                VecDeque::from(vec![0, 0, 0, 0, 0]),
            ]
        );
    }

    #[test]
    fn test_compute_history_left() {
        let input = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(
            compute_history_left(&input),
            vec![
                VecDeque::from(vec![-3, 0, 3, 6, 9, 12, 15]),
                VecDeque::from(vec![3, 3, 3, 3, 3, 3]),
                VecDeque::from(vec![0, 0, 0, 0, 0]),
            ]
        );
    }

    #[test]
    fn test_compute_history_left_2() {
        let input = vec![1, 3, 6, 10, 15, 21];
        assert_eq!(
            compute_history_left(&input),
            vec![
                VecDeque::from(vec![0, 1, 3, 6, 10, 15, 21]),
                VecDeque::from(vec![1, 2, 3, 4, 5, 6]),
                VecDeque::from(vec![1, 1, 1, 1, 1]),
                VecDeque::from(vec![0, 0, 0, 0]),
            ]
        );
    }

    #[test]
    fn test_part2() {
        let input = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;
        assert_eq!(part2(input), 2);
    }
}
