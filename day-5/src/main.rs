use std::{fs::File, io::Read, str::FromStr};

fn convert_seed(destination: u64, source: u64, delta: u64, input: u64) -> Option<u64> {
    match input.checked_sub(source) {
        Some(x) => (x < delta).then_some(destination + (input - source)),
        None => None,
    }
}

fn convert_seed_range(
    destination: u64,
    source: u64,
    delta: u64,
    start: u64,
    end: u64,
) -> (Option<Vec<Vec<u64>>>, Option<Vec<Vec<u64>>>) {
    if end < source || start >= source + delta {
        return (Some(vec![vec![start, end]]), None);
    }

    if start >= source && end <= source + delta {
        return (
            None,
            Some(vec![vec![
                destination + (start - source),
                destination + (end - source),
            ]]),
        );
    }

    if start >= source {
        return (
            Some(vec![vec![source + delta, end]]),
            Some(vec![vec![
                destination + (start - source),
                destination + delta,
            ]]),
        );
    }

    if end <= source + delta {
        return (
            Some(vec![vec![start, source]]),
            Some(vec![vec![destination, destination + (end - source)]]),
        );
    }

    if start < source && end > source + delta {
        return (
            Some(vec![vec![start, source], vec![source + delta, end]]),
            Some(vec![vec![destination, destination + delta]]),
        );
    }

    dbg!(destination, source, delta, start, end);
    unreachable!()
}

#[derive(Debug)]
struct Maps {
    seeds: Vec<u64>,
    conversions: Vec<Vec<Vec<u64>>>,
}

#[derive(Debug)]
struct ParseMapError;

impl FromStr for Maps {
    type Err = ParseMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sections = s.split("\r\n\r\n");

        let seeds: Vec<u64> = sections
            .next()
            .unwrap()
            .strip_prefix("seeds: ")
            .unwrap()
            .split_whitespace()
            .map(|d| {
                d.parse()
                    .unwrap_or_else(|_| panic!("Cannot parse value: {:?}", d))
            })
            .collect();

        let mut conversions = Vec::new();
        for section in sections {
            let mut section_vec: Vec<Vec<u64>> = Vec::new();
            for line in section.lines().skip(1) {
                section_vec.push(
                    line.split_whitespace()
                        .map(|d| d.parse().unwrap())
                        .collect(),
                );
            }
            conversions.push(section_vec);
        }

        Ok(Self { seeds, conversions })
    }
}

fn part_1(input: &str) -> u64 {
    let maps: Maps = input.parse().unwrap();

    let mut result: u64 = u64::MAX;

    for mut seed in maps.seeds {
        for con in &maps.conversions {
            for mapping in con {
                if let Some(x) = convert_seed(mapping[0], mapping[1], mapping[2], seed) {
                    seed = x;
                    break;
                }
            }
        }
        result = result.min(seed);
    }

    result
}

fn part_2(input: &str) -> u64 {
    let maps: Maps = input.parse().unwrap();

    let mut result: u64 = u64::MAX;

    for seed_range in maps.seeds.chunks_exact(2) {
        let (seed_start, seed_end) = (seed_range[0], seed_range[0] + seed_range[1]);

        let mut set_values = Vec::new();
        let mut values = vec![vec![seed_start, seed_end]];
        for con in &maps.conversions {
            let mut new_values = Vec::new();
            for mapping in con {
                for set in &values {
                    let (stopped, ranges) =
                        convert_seed_range(mapping[0], mapping[1], mapping[2], set[0], set[1]);

                    if let Some(stopped) = stopped {
                        set_values.extend(stopped);
                    }
                    if let Some(ranges) = ranges {
                        new_values.extend(ranges);
                    }
                }
            }
            values = new_values;
        }
        result = result.min(*set_values.iter().flatten().min().unwrap());
    }

    result
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

    const INPUT: &str = "seeds: 79 14 55 13\r
\r
seed-to-soil map:
50 98 2
52 50 48\r
\r
soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15\r
\r
fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4\r
\r
water-to-light map:
88 18 7
18 25 70\r
\r
light-to-temperature map:
45 77 23
81 45 19
68 64 13\r
\r
temperature-to-humidity map:
0 69 1
1 0 69\r
\r
humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_conversion() {
        assert_eq!(convert_seed(50, 98, 2, 97), None);
        assert_eq!(convert_seed(50, 98, 2, 98), Some(50));
        assert_eq!(convert_seed(50, 98, 2, 99), Some(51));
        assert_eq!(convert_seed(50, 98, 2, 100), None);
    }

    #[test]
    fn test_parse() {
        dbg!(INPUT.parse::<Maps>().unwrap());
    }

    #[test]
    fn test_part_2_parse() {
        //dbg!(part2map(input).unwrap());
    }

    #[test]
    fn test_range_comparisons() {}

    #[test]
    fn p1() {
        assert_eq!(part_1(INPUT), 35)
    }

    #[test]
    fn p2() {
        assert_eq!(part_2(INPUT), 46)
    }
}
