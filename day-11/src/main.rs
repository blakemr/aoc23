use itertools::Itertools;
use std::{fs::File, io::Read};

fn get_expansions(s: &str) -> (Vec<usize>, Vec<usize>) {
    let mut rows = Vec::new();
    let mut columns = Vec::new();
    let mut mask = s.lines().next().unwrap().to_string();

    s.lines().enumerate().for_each(|(i, line)| {
        mask = mask
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if c == '#' || line.as_bytes()[i] as char == '#' {
                    '#'
                } else {
                    '.'
                }
            })
            .collect();

        if line.chars().all(|c| c == '.') {
            rows.push(i)
        }
    });

    mask.chars().enumerate().for_each(|(i, m)| {
        if m == '.' {
            columns.push(i)
        }
    });

    (rows, columns)
}

fn galaxies(s: &str, ex: usize) -> Vec<(usize, usize)> {
    let mut g = Vec::new();
    let space = get_expansions(s);

    s.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            let r_ex = space.0.iter().filter(|n| n < &&i).count() * ex;
            let c_ex = space.1.iter().filter(|n| n < &&j).count() * ex;
            if c == '#' {
                g.push((i + r_ex, j + c_ex))
            }
        })
    });

    g
}

fn distance_sum(input: &str, expansion_factor: usize) -> usize {
    galaxies(input, expansion_factor - 1)
        .iter()
        .combinations(2)
        .map(|pair| {
            let a = pair[0];
            let b = pair[1];

            a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
        })
        .sum()
}

fn part_1(input: &str) -> usize {
    distance_sum(input, 2)
}

fn part_2(input: &str) -> usize {
    distance_sum(input, 1_000_000)
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

    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_find() {
        dbg!(galaxies(INPUT, 1));
    }

    #[test]
    fn p1() {
        assert_eq!(part_1(INPUT), 374);
    }

    #[test]
    fn p2() {
        assert_eq!(distance_sum(INPUT, 10), 1030);
        assert_eq!(distance_sum(INPUT, 100), 8410);
    }
}
