use std::{fs::File, io::Read, string::ParseError};

fn quadratic(a: f64, b: f64, c: f64) -> (f64, f64) {
    let s = (b * b - 4. * a * c).sqrt();
    ((-b + s) / (2. * a), (-b - s) / (2. * a))
}

#[derive(Debug)]
struct Race {
    time: usize,
    record: usize,
}

impl Race {
    fn ways_to_win(&self) -> usize {
        let limits = quadratic(-1., self.time as f64, -(self.record as f64));

        // Special case: we don't want to match the record
        let left = (limits.0.fract() == 0.) as usize;
        let right = (limits.1.fract() == 0.) as usize;

        // The +1 is to be inclusive of the ceiling round
        limits.1.floor() as usize - limits.0.ceil() as usize - left - right + 1
    }
}

fn read_races(s: &str) -> Result<Vec<Race>, ParseError> {
    let mut times: Vec<usize> = Vec::new();
    let mut records: Vec<usize> = Vec::new();

    s.lines()
        .next()
        .unwrap()
        .strip_prefix("Time: ")
        .unwrap()
        .split_whitespace()
        .for_each(|n| times.push(n.parse().unwrap()));
    s.lines()
        .nth(1)
        .unwrap()
        .strip_prefix("Distance: ")
        .unwrap()
        .split_whitespace()
        .for_each(|n| records.push(n.parse().unwrap()));

    Ok(times
        .iter()
        .zip(records)
        .map(|(time, record)| Race {
            time: *time,
            record,
        })
        .collect())
}

fn read_frag_race(s: &str) -> Result<Race, ParseError> {
    let time: usize = s
        .lines()
        .next()
        .unwrap()
        .strip_prefix("Time: ")
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .concat()
        .parse()
        .unwrap();
    let record = s
        .lines()
        .nth(1)
        .unwrap()
        .strip_prefix("Distance: ")
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .concat()
        .parse()
        .unwrap();

    Ok(Race { time, record })
}

fn part_1(input: &str) -> usize {
    let races = read_races(input).unwrap();
    let mut mul = 1;
    for race in races {
        mul *= race.ways_to_win();
    }

    mul
}

fn part_2(input: &str) -> usize {
    read_frag_race(input).unwrap().ways_to_win()
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

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn check_parse() {
        dbg!(read_races(INPUT));
    }

    #[test]
    fn check_parse_2() {
        dbg!(read_frag_race(INPUT));
    }

    #[test]
    fn check_wins() {
        let races = read_races(INPUT).unwrap();
        dbg!(races[0].ways_to_win());
    }

    #[test]
    fn p1() {
        assert_eq!(part_1(INPUT), 288);
    }

    #[test]
    fn p2() {
        assert_eq!(part_2(INPUT), 71503);
    }
}
