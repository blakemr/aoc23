use std::{fs::File, io::Read, str::FromStr};

#[derive(Debug)]
struct Bag {
    idx: u32,
    red: u32,
    green: u32,
    blue: u32,
}

impl Bag {
    fn can_hold(&self, r: u32, g: u32, b: u32) -> bool {
        self.red <= r && self.green <= g && self.blue <= b
    }

    fn power(&self) -> u32 {
        self.red * self.blue * self.green
    }
}

#[derive(Debug)]
struct BagParseError;

impl FromStr for Bag {
    type Err = BagParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (idx, games) = s.split_once(':').unwrap();

        let (_, idx) = idx
            .split_once("Game ")
            .unwrap_or_else(|| panic!("Failed to split \"{idx}\" from: {idx}, {games}"));
        let idx: u32 = idx.parse().unwrap();

        let games = games.split(';');
        let (mut red, mut green, mut blue) = (0, 0, 0);
        for game in games {
            let colors = game.split(',');
            for color in colors {
                let (n, c) = color.trim().split_once(' ').unwrap();
                match c {
                    "red" => red = red.max(n.parse().unwrap()),
                    "green" => green = green.max(n.parse().unwrap()),
                    "blue" => blue = blue.max(n.parse().unwrap()),
                    s => panic!("Unknown input: {}", s),
                }
            }
        }

        Ok(Bag {
            idx,
            red,
            green,
            blue,
        })
    }
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

fn part_1(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let b: Bag = line.parse().unwrap();
        if b.can_hold(12, 13, 14) {
            sum += b.idx;
        }
    }

    sum
}

fn part_2(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let b: Bag = line.parse().unwrap();
        sum += b.power();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let test_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(part_1(test_input), 8);
    }

    #[test]
    fn p2() {
        let test_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(part_2(test_input), 2286);
    }
}
