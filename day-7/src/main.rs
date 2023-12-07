use std::{collections::HashMap, fs::File, io::Read, str::FromStr};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    strength: u8,
    cards: [u8; 5],
    bid: usize,
}

#[derive(Debug)]
struct HandParseError;

impl FromStr for Hand {
    type Err = HandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.split_once(' ').unwrap();

        let bid = bid.parse().unwrap();

        let cards: [u8; 5] = hand
            .chars()
            .map(|c| match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 1,
                'T' => 10,
                n => n.to_digit(10).unwrap() as u8,
            })
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();

        let mut strength_hash = HashMap::new();
        let mut j_count = 0;
        cards.iter().for_each(|c| {
            if *c == 1 {
                j_count += 1;
            } else if strength_hash.contains_key(c) {
                strength_hash.insert(c, strength_hash[c] + 1);
            } else {
                strength_hash.insert(c, 1);
            }
        });

        for key in strength_hash.clone().keys() {
            strength_hash.insert(*key, strength_hash[key] + j_count);
        }

        let strength = if strength_hash.len() <= 1 {
            7
        } else if strength_hash.len() == 2 && strength_hash.values().any(|v| *v == 4) {
            6
        } else if strength_hash.len() == 2 {
            5
        } else if strength_hash.len() == 3 && strength_hash.values().any(|v| *v == 3) {
            4
        } else if strength_hash.len() == 3 {
            3
        } else if strength_hash.len() == 4 {
            2
        } else {
            1
        };

        Ok(Hand {
            strength,
            cards,
            bid,
        })
    }
}

fn part_1(input: &str) -> u32 {
    let mut hands: Vec<Hand> = Vec::new();

    for line in input.lines() {
        hands.push(line.parse().unwrap());
    }
    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i + 1))
        .sum::<usize>() as u32
}

fn part_2(input: &str) -> u32 {
    part_1(input)
}

fn main() {
    let mut text = String::new();
    File::open("input.txt")
        .expect("Failed to open file.")
        .read_to_string(&mut text)
        .expect("Failed to read file.");

    dbg!(part_1(text.as_str()));
    dbg!(part_2(text.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_parse() {
        for line in INPUT.lines() {
            dbg!(line.parse::<Hand>().unwrap());
        }
    }

    #[test]
    fn test_sort() {
        let mut hands: Vec<Hand> = Vec::new();

        for line in INPUT.lines() {
            hands.push(line.parse().unwrap());
        }
        hands.sort();
        dbg!(hands);
    }

    #[test]
    fn p1() {
        assert_eq!(part_1(INPUT), 6440);
    }

    #[test]
    fn p2() {
        assert_eq!(part_2(INPUT), 5905);
    }
}
