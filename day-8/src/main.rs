use num::integer::lcm;
use std::{collections::HashMap, fs::File, io::Read, str::FromStr, string::ParseError};

#[derive(Debug)]
struct Map {
    directions: Vec<char>,
    elements: HashMap<String, (String, String)>,
    distances: HashMap<String, (usize, Vec<usize>)>,
}

impl Map {
    fn steps(&self, start: &String, end: &String) -> u32 {
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
                'L' => current = &self.elements[current].0,
                'R' => current = &self.elements[current].1,
                _ => panic!(),
            }
            idx = (idx + 1) % self.directions.len();
        }

        steps
    }

    fn steps_to_suffix(&self, start: &String, end: &char) -> u32 {
        let mut steps = 0;
        let mut idx = 0;
        let mut current = start;
        loop {
            // Check if we're there
            if current.ends_with(*end) {
                break;
            }
            steps += 1;
            // Update instruction and elem
            match self.directions[idx] {
                'L' => current = &self.elements[current].0,
                'R' => current = &self.elements[current].1,
                _ => panic!(),
            }
            dbg!(current);
            idx = (idx + 1) % self.directions.len();
        }

        steps
    }

    fn set_distances(&self, start: &char, end: &char) -> HashMap<String, (usize, Vec<usize>)> {
        self.elements
            .keys()
            .filter(|k| k.ends_with(*start))
            .cloned()
            .map(|k| {
                let mut k_vec = Vec::new();
                let mut visited = Vec::new();
                let mut current = k.clone();
                let mut idx = 0;
                let mut steps = 0;

                loop {
                    current = match self.directions[idx] {
                        'L' => self.elements[&current].0.clone(),
                        'R' => self.elements[&current].1.clone(),
                        _ => unreachable!(),
                    };
                    steps += 1;
                    if current.ends_with(*end) {
                        k_vec.push(steps);
                        steps = 0;
                    }

                    if visited.contains(&(current.clone(), idx)) {
                        break;
                    } else {
                        visited.push((current.clone(), idx));
                    }

                    idx = (idx + 1) % self.directions.len();
                }

                (
                    k,
                    (
                        visited
                            .iter()
                            .position(|x| x == &(current.clone(), idx))
                            .unwrap(),
                        k_vec,
                    ),
                )
            })
            .collect()
    }

    fn ghost_steps(&self, start: &char, end: &char) -> usize {
        todo!();

        // Debug output has shown I can just use LCM

        //        let mut steps = 0;
        //let mut idx = 0;
        //let mut currents: Vec<String> = self
        //.distances
        //.keys()
        //.filter(|k| k.ends_with(*start))
        //.cloned()
        //.collect();

        //loop {
        //if currents.iter().all(|loc| loc.ends_with(*end)) {
        //break;
        //}

        //let next_match = currents
        //.iter()
        //.map(|s| match self.directions[idx] {
        //'L' => self.distances[s].0,
        //'R' => self.distances[s].1,
        //_ => unreachable!(),
        //})
        //.min()
        //.unwrap();

        //dbg!(next_match);

        //steps += next_match;

        //for _ in 0..next_match {
        //currents = currents
        //.iter()
        //.map(|elem| match self.directions[idx] {
        //'L' => self.elements[elem].0.clone(),
        //'R' => self.elements[elem].1.clone(),
        //_ => unreachable!(),
        //})
        //.collect();

        //idx = (idx + 1) % self.directions.len();
        //}
        //}

        //steps
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

        let mut m = Self {
            directions,
            elements,
            distances: HashMap::new(),
        };

        m.distances = m.set_distances(&'A', &'Z');

        Ok(m)
    }
}

fn part_1(input: &str) -> u32 {
    input
        .parse::<Map>()
        .unwrap()
        .steps(&"AAA".to_string(), &"ZZZ".to_string())
}

fn part_2(input: &str) -> isize {
    let binding = input.parse::<Map>().unwrap();
    let d = binding.distances.iter().map(|(_, (_, v))| v[0] as isize);

    let mut l = 1;
    d.for_each(|n| l = lcm(l, n));

    l
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
        dbg!(INPUT_2.parse::<Map>());
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
