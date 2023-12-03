use std::{collections::HashMap, fs::File, io::Read, str::FromStr};

fn split_with_indicies(s: &str, pat: char) -> impl Iterator<Item = (usize, &str)> {
    s.split(pat)
        .flat_map(|sub| sub.split(|c: char| !c.is_numeric()))
        .map(move |sub| (sub.as_ptr() as usize - s.as_ptr() as usize, sub))
        .filter(|(_, s)| !s.is_empty())
        .chain(
            s.split(pat)
                .flat_map(|sub| sub.split(|c: char| c.is_numeric()))
                .map(move |sub| (sub.as_ptr() as usize - s.as_ptr() as usize, sub))
                .filter(|(_, s)| !s.is_empty()),
        )
}

#[derive(Debug, Clone)]
struct Part {
    value: u32,
    indicies: Vec<(usize, usize)>,
}

impl Part {
    fn part_value(&self, symbols: &HashMap<(usize, usize), char>) -> u32 {
        for idx in self.indicies.clone().into_iter() {
            let xys = [
                (idx.0.saturating_sub(1), idx.1.saturating_sub(1)),
                (idx.0.saturating_sub(1), idx.1),
                (idx.0.saturating_sub(1), idx.1 + 1),
                (idx.0, idx.1.saturating_sub(1)),
                (idx.0, idx.1 + 1),
                (idx.0 + 1, idx.1.saturating_sub(1)),
                (idx.0 + 1, idx.1),
                (idx.0 + 1, idx.1 + 1),
            ];
            for xy in xys {
                if symbols.get(&xy).is_some() {
                    return self.value;
                }
            }
        }

        0
    }

    fn adjacent_gears(
        &self,
        symbols: &HashMap<(usize, usize), char>,
        gears: &mut HashMap<(usize, usize), Vec<u32>>,
    ) {
        for idx in self.indicies.clone().into_iter() {
            let xys = [
                (idx.0.saturating_sub(1), idx.1.saturating_sub(1)),
                (idx.0.saturating_sub(1), idx.1),
                (idx.0.saturating_sub(1), idx.1 + 1),
                (idx.0, idx.1.saturating_sub(1)),
                (idx.0, idx.1 + 1),
                (idx.0 + 1, idx.1.saturating_sub(1)),
                (idx.0 + 1, idx.1),
                (idx.0 + 1, idx.1 + 1),
            ];
            for xy in xys {
                if let Some(sym) = symbols.get(&xy) {
                    if *sym == '*' {
                        match gears.get_mut(&xy) {
                            Some(v) => {
                                v.push(self.value);
                            }
                            None => {
                                gears.insert(xy, vec![self.value]);
                            }
                        }
                        return;
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Piece {
    Part(Part),
    Symbol(char),
}

impl Piece {
    fn new(s: &str, idx: (usize, usize)) -> Option<Self> {
        let mut indicies = Vec::new();
        for i in 0..s.len() {
            indicies.push((idx.0, idx.1 + i));
        }

        let piece;
        if let Ok(value) = s.parse() {
            piece = Piece::Part(Part { value, indicies });
        } else {
            if s.len() > 1 {
                return None;
            }
            piece = Piece::Symbol(s.chars().next().unwrap());
        }

        Some(piece)
    }
}

struct Schematic {
    parts: HashMap<(usize, usize), Part>,
    symbols: HashMap<(usize, usize), char>,
}

#[derive(Debug)]
struct SchematicParseError;

impl FromStr for Schematic {
    type Err = SchematicParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = HashMap::new();
        let mut symbols = HashMap::new();
        for (i, line) in s.lines().enumerate() {
            for (j, sub) in split_with_indicies(line, '.') {
                if let Some(piece) = Piece::new(sub, (i, j)) {
                    match piece {
                        Piece::Part(p) => {
                            parts.insert((i, j), p);
                        }
                        Piece::Symbol(c) => {
                            symbols.insert((i, j), c);
                        }
                    };
                }
            }
        }

        Ok(Self { parts, symbols })
    }
}

fn part_1(input: &str) -> u32 {
    let schem: Schematic = input.parse().unwrap();

    let mut sum = 0;
    for (_, v) in schem.parts {
        sum += v.part_value(&schem.symbols);
    }

    sum
}

fn part_2(input: &str) -> u32 {
    let schem: Schematic = input.parse().unwrap();

    let mut sum = 0;
    let mut gears = HashMap::new();
    for (_, v) in schem.parts {
        v.adjacent_gears(&schem.symbols, &mut gears);
    }

    for (_, v) in gears {
        if v.len() == 2 {
            sum += v.iter().product::<u32>();
        }
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
    fn test_split() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$/*....
.664.598..";
        for line in input.lines() {
            println!("{:?}", split_with_indicies(line, '.').collect::<Vec<_>>());
        }
    }

    #[test]
    fn p1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(part_1(input), 4361);
    }

    #[test]
    fn p2() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(part_2(input), 467835);
    }
}
