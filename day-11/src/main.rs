use itertools::Itertools;
use std::{fs::File, io::Read};

fn expand(s: &str) -> String {
    let mut ex = s.to_string();
    let mut mask = s.lines().next().unwrap().to_string();

    // Expand rows
    ex = ex
        .lines()
        .map(|line| {
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
                line.to_owned() + "\n" + line + "\n"
            } else {
                line.to_owned() + "\n"
            }
        })
        .collect();

    // Expand Columns
    ex.lines()
        .map(|line| {
            mask.chars()
                .zip(line.chars())
                .map(|(m, c)| {
                    if m == '#' {
                        c.to_string()
                    } else {
                        "..".to_string()
                    }
                })
                .collect::<String>()
                + "\n"
        })
        .collect()
}

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

fn part_1(input: &str) -> usize {
    galaxies(input, 1)
        .iter()
        .combinations(2)
        .map(|pair| {
            let a = pair[0];
            let b = pair[1];

            a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    galaxies(input, 1_000_000 - 1)
        .iter()
        .combinations(2)
        .map(|pair| {
            let a = pair[0];
            let b = pair[1];

            a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
        })
        .sum()
}

fn part_n(input: &str, n: usize) -> usize {
    galaxies(input, n)
        .iter()
        .combinations(2)
        .map(|pair| {
            let a = pair[0];
            let b = pair[1];

            a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
        })
        .sum()
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

    const INPUT_EXP: &str = "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......
";

    #[test]
    fn test_expand() {
        assert_eq!(expand(INPUT), INPUT_EXP);
    }

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
        assert_eq!(part_n(INPUT, 10 - 1), 1030);
        assert_eq!(part_n(INPUT, 100 - 1), 8410);
    }
}
