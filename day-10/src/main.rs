use std::{collections::HashSet, fs::File, io::Read, str::FromStr, string::ParseError};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
struct Maze {
    map: Vec<Vec<char>>,
    start: (usize, usize),
}

impl Maze {
    fn loop_size(&self) -> usize {
        let mut position = self.start;
        let mut size = 0;
        let mut direction;

        if "F-L".contains(self.map[position.0][position.1 - 1]) {
            direction = Direction::Left;
        } else if "7-J".contains(self.map[position.0][position.1]) {
            direction = Direction::Right;
        } else {
            direction = Direction::Down;
        }

        loop {
            match direction {
                Direction::Left => position.1 -= 1,
                Direction::Right => position.1 += 1,
                Direction::Up => position.0 -= 1,
                Direction::Down => position.0 += 1,
            }

            match self.map[position.0][position.1] {
                '|' | '-' => {}
                'L' => {
                    if direction == Direction::Down {
                        direction = Direction::Right;
                    } else {
                        direction = Direction::Up;
                    }
                }
                'J' => {
                    if direction == Direction::Down {
                        direction = Direction::Left;
                    } else {
                        direction = Direction::Up;
                    }
                }
                '7' => {
                    if direction == Direction::Up {
                        direction = Direction::Left;
                    } else {
                        direction = Direction::Down;
                    }
                }
                'F' => {
                    if direction == Direction::Up {
                        direction = Direction::Right;
                    } else {
                        direction = Direction::Down;
                    }
                }
                'S' => return size + 1,
                _ => {
                    dbg!(self.map[position.0][position.1]);
                    panic!();
                }
            }

            //dbg!(&direction, self.map[position.0][position.1], position);
            size += 1;
        }
    }

    fn get_loop(&self) -> Vec<(usize, usize)> {
        let mut position = self.start;
        let mut ring = vec![position];
        let mut direction;

        if "F-L".contains(self.map[position.0][position.1 - 1]) {
            direction = Direction::Left;
        } else if "7-J".contains(self.map[position.0][position.1]) {
            direction = Direction::Right;
        } else {
            direction = Direction::Down;
        }

        loop {
            match direction {
                Direction::Left => position.1 -= 1,
                Direction::Right => position.1 += 1,
                Direction::Up => position.0 -= 1,
                Direction::Down => position.0 += 1,
            }

            match self.map[position.0][position.1] {
                '|' | '-' => {}
                'L' => {
                    if direction == Direction::Down {
                        direction = Direction::Right;
                    } else {
                        direction = Direction::Up;
                    }
                }
                'J' => {
                    if direction == Direction::Down {
                        direction = Direction::Left;
                    } else {
                        direction = Direction::Up;
                    }
                }
                '7' => {
                    if direction == Direction::Up {
                        direction = Direction::Left;
                    } else {
                        direction = Direction::Down;
                    }
                }
                'F' => {
                    if direction == Direction::Up {
                        direction = Direction::Right;
                    } else {
                        direction = Direction::Down;
                    }
                }
                'S' => return ring,
                _ => {
                    dbg!(self.map[position.0][position.1]);
                    panic!();
                }
            }

            //dbg!(&direction, self.map[position.0][position.1], position);
            ring.push(position);
        }
    }

    fn get_directions(&self) -> Vec<Direction> {
        let mut position = self.start;
        let mut direction;

        if "F-L".contains(self.map[position.0][position.1 - 1]) {
            direction = Direction::Left;
        } else if "7-J".contains(self.map[position.0][position.1]) {
            direction = Direction::Right;
        } else {
            direction = Direction::Down;
        }
        let mut ring = vec![direction];

        loop {
            match direction {
                Direction::Left => position.1 -= 1,
                Direction::Right => position.1 += 1,
                Direction::Up => position.0 -= 1,
                Direction::Down => position.0 += 1,
            }

            match self.map[position.0][position.1] {
                '|' | '-' => {}
                'L' => {
                    if direction == Direction::Down {
                        direction = Direction::Right;
                    } else {
                        direction = Direction::Up;
                    }
                }
                'J' => {
                    if direction == Direction::Down {
                        direction = Direction::Left;
                    } else {
                        direction = Direction::Up;
                    }
                }
                '7' => {
                    if direction == Direction::Up {
                        direction = Direction::Left;
                    } else {
                        direction = Direction::Down;
                    }
                }
                'F' => {
                    if direction == Direction::Up {
                        direction = Direction::Right;
                    } else {
                        direction = Direction::Down;
                    }
                }
                'S' => return ring,
                _ => {
                    dbg!(self.map[position.0][position.1]);
                    panic!();
                }
            }

            //dbg!(&direction, self.map[position.0][position.1], position);
            ring.push(direction);
        }
    }

    fn get_blanks(&self) -> HashSet<(usize, usize)> {
        let mut position = self.start;
        let mut blanks = HashSet::new();
        let mut direction;

        if "F-L".contains(self.map[position.0][position.1 - 1]) {
            direction = Direction::Left;
        } else if "7-J".contains(self.map[position.0][position.1]) {
            direction = Direction::Right;
        } else {
            direction = Direction::Down;
        }

        loop {
            if position.0 > 0 {
                blanks.insert((position.0 - 1, position.1));
            }
            if position.0 + 1 < self.map.len() {
                blanks.insert((position.0 + 1, position.1));
            }
            if position.1 > 0 {
                blanks.insert((position.0, position.1 - 1));
            }
            if position.1 + 1 > self.map[0].len() {
                blanks.insert((position.0, position.1 + 1));
            }

            match direction {
                Direction::Left => position.1 -= 1,
                Direction::Right => position.1 += 1,
                Direction::Up => position.0 -= 1,
                Direction::Down => position.0 += 1,
            }

            match self.map[position.0][position.1] {
                '|' | '-' => {}
                'L' => {
                    if direction == Direction::Down {
                        direction = Direction::Right;
                    } else {
                        direction = Direction::Up;
                    }
                }
                'J' => {
                    if direction == Direction::Down {
                        direction = Direction::Left;
                    } else {
                        direction = Direction::Up;
                    }
                }
                '7' => {
                    if direction == Direction::Up {
                        direction = Direction::Left;
                    } else {
                        direction = Direction::Down;
                    }
                }
                'F' => {
                    if direction == Direction::Up {
                        direction = Direction::Right;
                    } else {
                        direction = Direction::Down;
                    }
                }
                'S' => return blanks,
                _ => {
                    dbg!(self.map[position.0][position.1]);
                    panic!();
                }
            }
        }
    }

    fn check_void(
        &self,
        ring: &Vec<(usize, usize)>,
        counted: &HashSet<(usize, usize)>,
        current: &mut HashSet<(usize, usize)>,
        seed: (usize, usize),
    ) -> bool {
        if ring.contains(&seed) {
            return true;
        }

        if counted.contains(&seed) {
            return false;
        }

        if current.insert(seed) {
            let sides = [
                if seed.0 > 0 {
                    self.check_void(ring, counted, current, (seed.0 - 1, seed.1))
                } else {
                    false
                },
                if seed.0 + 1 < self.map.len() {
                    self.check_void(ring, counted, current, (seed.0 + 1, seed.1))
                } else {
                    false
                },
                if seed.1 > 0 {
                    self.check_void(ring, counted, current, (seed.0, seed.1 - 1))
                } else {
                    false
                },
                if seed.1 + 1 < self.map[0].len() {
                    self.check_void(ring, counted, current, (seed.0, seed.1 + 1))
                } else {
                    false
                },
            ];

            sides.iter().all(|f| *f)
        } else {
            true
        }
    }

    fn check_winding(
        &self,
        ring: &[(usize, usize)],
        directions: &[Direction],
        point: &(usize, usize),
    ) -> bool {
        let mut crosses = [0, 0, 0, 0];

        for (i, node) in ring.iter().enumerate() {
            if node.1 == point.1 && node.0 < point.0 {
                // node above
                match directions[i] {
                    Direction::Left => crosses[2] += 1,
                    Direction::Right => crosses[2] -= 1,
                    _ => {}
                }
            } else if node.1 == point.1 && node.0 > point.0 {
                // node below
                match directions[i] {
                    Direction::Left => crosses[3] -= 1,
                    Direction::Right => crosses[3] += 1,
                    _ => {}
                }
            }
            if node.0 == point.0 && node.1 < point.1 {
                // node left
                match directions[i] {
                    Direction::Up => crosses[0] += 1,
                    Direction::Down => crosses[0] -= 1,
                    _ => {}
                }
            } else if node.0 == point.0 && node.1 > point.1 {
                // node right
                match directions[i] {
                    Direction::Up => crosses[1] -= 1,
                    Direction::Down => crosses[1] += 1,
                    _ => {}
                }
            }
            //dbg!(crosses);
        }

        dbg!(crosses);
        crosses == [1, 1, 1, 1] || crosses == [-1, -1, -1, -1]
    }

    fn find_voids_in_loop(&self) -> usize {
        let ring = self.get_loop();
        let directions = self.get_directions();
        let mut counted = HashSet::new();
        let blanks: HashSet<(usize, usize)> = self
            .get_blanks()
            .iter()
            .filter(|elem| !ring.contains(elem))
            .copied()
            .collect();
        let mut voids = 0;

        for blank in blanks {
            let mut current = HashSet::new();
            if self.check_void(&ring, &counted, &mut current, blank) {
                current.retain(|c| self.check_winding(ring.as_slice(), directions.as_slice(), c));
                voids += current.len();
                dbg!(&current);
            }
            counted.extend(current);
        }

        voids
    }
}

impl FromStr for Maze {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = (0, 0);
        let map = s
            .lines()
            .enumerate()
            .map(|(i, line)| {
                if let Some(j) = line.find('S') {
                    start = (i, j);
                }
                line.chars().collect()
            })
            .collect();
        Ok(Self { map, start })
    }
}

fn part_1(input: &str) -> usize {
    input.parse::<Maze>().unwrap().loop_size() / 2
}

fn part_2(input: &str) -> usize {
    input.parse::<Maze>().unwrap().find_voids_in_loop()
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

    const INPUT: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

    const INPUT_2: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    const INPUT_3: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    const INPUT_4: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    #[test]
    fn test_parse() {
        dbg!(INPUT.parse::<Maze>().unwrap());
    }

    #[test]
    fn test_loop() {
        dbg!(INPUT.parse::<Maze>().unwrap().get_loop());
    }

    // Assumption: there's only 1 answer
    #[test]
    fn p1() {
        assert_eq!(part_1(INPUT), 4);
    }

    #[test]
    fn p2() {
        assert_eq!(part_2(INPUT_2), 4);
        assert_eq!(part_2(INPUT_4), 8);
        assert_eq!(part_2(INPUT_3), 10);
    }
}
