use std::{collections::HashMap, fs::File, io::Read, str::FromStr, string::ParseError};

#[derive(Debug)]
struct Map {
    directions: Vec<char>,
    elements: HashMap<String, (String, String)>,
}

impl Map {
    fn steps(&self, start: String, end: String) -> u32 {
        let mut steps = 0;
        let mut idx = 0;
        let mut current = start;
        loop {
            // Check if we're there
            if current == end {
                break;
            }
            steps += 1;
            // Update instruction and elem
            match self.directions[idx] {
                'L' => current = self.elements[&current].0.clone(),
                'R' => current = self.elements[&current].1.clone(),
                _ => panic!(),
            }
            idx = (idx + 1) % self.directions.len();
        }

        steps
    }

    fn ghost_steps(&self, start: char, end: char) -> u32 {
        let mut steps = 0;
        let mut idx = 0;
        let mut currents: Vec<String> = self
            .elements
            .keys()
            .filter(|k| k.ends_with(start))
            .cloned()
            .collect();

        loop {
            if currents.iter().all(|loc| loc.ends_with(end)) {
                break;
            }

            steps += 1;

            currents = currents
                .iter()
                .map(|elem| match self.directions[idx] {
                    'L' => self.elements[elem].0.clone(),
                    'R' => self.elements[elem].1.clone(),
                    _ => panic!(),
                })
                .collect();

            idx = (idx + 1) % self.directions.len();
        }

        steps
    }
}

impl FromStr for Map {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let directions: Vec<char> = s.lines().next().unwrap().chars().collect();
        let mut elements = HashMap::new();
        s.lines().skip(2).for_each(|line| {
            let (k, v) = line.split_once(" = ").unwrap();
            let v = v
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .split_once(", ")
                .unwrap();
            elements.insert(k.to_string(), (v.0.to_string(), v.1.to_string()));
        });

        Ok(Self {
            directions,
            elements,
        })
    }
}

fn part_1(input: &str) -> u32 {
    input
        .parse::<Map>()
        .unwrap()
        .steps("AAA".to_string(), "ZZZ".to_string())
}

fn part_2(input: &str) -> u32 {
    input.parse::<Map>().unwrap().ghost_steps('A', 'Z')
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

    const INPUT: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const ALT_INPUT: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const INPUT_2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_parse() {
        dbg!(INPUT.parse::<Map>());
    }

    #[test]
    fn p1() {
        assert_eq!(part_1(INPUT), 2);
        assert_eq!(part_1(ALT_INPUT), 6);
    }

    #[test]
    fn p2() {
        assert_eq!(part_2(INPUT_2), 6);
    }
}
