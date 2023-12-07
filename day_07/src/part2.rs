use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Card {
    Joker,
    Number(u8),
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for Card {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'J' => Ok(Card::Joker),
            '2'..='9' => Ok(Card::Number(c.to_digit(10).unwrap() as u8)),
            'T' => Ok(Card::Number(10)),
            'Q' => Ok(Card::Queen),
            'K' => Ok(Card::King),
            'A' => Ok(Card::Ace),
            _ => Err(format!("Invalid card: {}", c)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    origin: String,
    count: BTreeMap<Card, u8>,
}

impl Hand {
    fn hand_type(&self) -> HandType {
        if self.count.len() == 1 {
            return HandType::FiveOfAKind;
        }

        let mut count = self.count.clone();
        let jokers_number = count.remove(&Card::Joker);

        let mut count = count.iter().collect::<Vec<_>>();
        count.sort_by(|&(c1, n1), &(c2, n2)| n2.cmp(n1).then(c2.cmp(c1)));

        let new_number = count[0].1 + jokers_number.unwrap_or(0);
        count[0] = (count[0].0, &new_number);

        match &count[..] {
            [(_, 5)] => HandType::FiveOfAKind,
            [(_, 4), _] => HandType::FourOfAKind,
            [(_, 3), (_, 2)] => HandType::FullHouse,
            [(_, 3), ..] => HandType::ThreeOfAKind,
            [(_, 2), (_, 2), _] => HandType::TwoPairs,
            [(_, 2), ..] => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

impl TryFrom<&str> for Hand {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.len() != 5 {
            return Err(format!("Invalid hand: {}", s));
        }
        Ok(Hand {
            origin: s.to_string(),
            count: s.chars().map(|c| Card::try_from(c).unwrap()).fold(
                BTreeMap::new(),
                |mut acc, card| {
                    *acc.entry(card).or_insert(0) += 1;
                    acc
                },
            ),
        })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            std::cmp::Ordering::Equal => {
                let mut o = self.origin.chars();
                let mut o2 = other.origin.chars();
                while let (Some(c1), Some(c2)) = (o.next(), o2.next()) {
                    match Card::try_from(c1)
                        .unwrap()
                        .cmp(&Card::try_from(c2).unwrap())
                    {
                        std::cmp::Ordering::Equal => continue,
                        o => return o,
                    }
                }
                std::cmp::Ordering::Equal
            }
            o => o,
        }
    }
}

fn parse_input(input: &str) -> Vec<(Hand, usize)> {
    input
        .lines()
        .map(|l| l.split(' '))
        .collect::<Vec<_>>()
        .iter_mut()
        .map(|h| {
            (
                Hand::try_from(h.next().unwrap()).unwrap(),
                h.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>()
}

pub fn part2(input: &str) -> usize {
    let mut hands_and_bids = parse_input(input);
    hands_and_bids.sort_by(|(h1, _1), (h2, _)| h1.cmp(h2));
    hands_and_bids
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) * bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structs() {
        let joker = Card::try_from('J').unwrap();
        let two = Card::try_from('2').unwrap();
        assert_eq!(joker, Card::Joker);
        assert!(joker < two);

        assert_eq!(HandType::OnePair, HandType::OnePair);
        assert!(HandType::OnePair < HandType::TwoPairs);
        assert!(Hand::try_from("32T3K").unwrap().hand_type() == HandType::OnePair);
        assert!(Hand::try_from("33332").unwrap() > Hand::try_from("2AAAA").unwrap());
        assert!(Hand::try_from("A2457").unwrap() > Hand::try_from("A2456").unwrap());

        assert!(Hand::try_from("32J3J").unwrap().hand_type() == HandType::FourOfAKind);
        assert_eq!(
            Hand::try_from("JKKK2").unwrap().hand_type(),
            HandType::FourOfAKind
        );
        assert!(Hand::try_from("JKKK2").unwrap() < Hand::try_from("QQQQ2").unwrap());
    }

    #[test]
    fn test_part2() {
        let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
        assert_eq!(part2(input), 5905);
    }
}
