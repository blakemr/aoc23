use std::{fs::File, io::Read, str::FromStr};

#[derive(Debug, Clone)]
struct Card {
    winners: Vec<u32>,
    recieved: Vec<u32>,
}

impl Card {
    fn matches(&self) -> usize {
        self.recieved
            .iter()
            .filter(|n| self.winners.contains(n))
            .count()
    }

    fn score(&self) -> u32 {
        let matches = self
            .recieved
            .iter()
            .filter(|n| self.winners.contains(n))
            .count();

        match matches {
            0 => 0,
            n => 1 << (n - 1),
        }
    }
}

#[derive(Debug)]
struct CardParseErr;

impl FromStr for Card {
    type Err = CardParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (w, r) = s.split_once(':').unwrap().1.split_once('|').unwrap();

        let winners: Vec<u32> = w.split_whitespace().map(|n| n.parse().unwrap()).collect();
        let recieved: Vec<u32> = r.split_whitespace().map(|n| n.parse().unwrap()).collect();

        Ok(Self { winners, recieved })
    }
}

fn part_1(input: &str) -> u32 {
    let mut cards = Vec::new();

    for line in input.lines() {
        let new_card: Card = line.parse().unwrap();

        cards.push(new_card);
    }

    cards.iter().fold(0, |acc, card| acc + card.score())
}

fn part_2(input: &str) -> u32 {
    let mut cards = Vec::new();

    for line in input.lines() {
        let new_card: Card = line.parse().unwrap();

        cards.push(new_card);
    }
    let mut counts: Vec<u32> = Vec::new();
    counts.resize(cards.len(), 1);

    let mut sum = 0;
    for (i, card) in cards.iter().enumerate() {
        let copies = counts[i];
        sum += copies;

        let matches = card.matches();
        counts[i + 1..i + 1 + matches].iter_mut().for_each(|c| {
            *c += copies;
        });
    }

    sum
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

    #[test]
    fn test_shifting() {
        let x: u32 = 2;

        dbg!(x << 1);
    }

    #[test]
    fn p1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(part_1(input), 13);
    }

    #[test]
    fn p2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(part_2(input), 30);
    }
}
