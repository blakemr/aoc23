use std::{fs::File, io::Read, string::ParseError};

struct Pascal {
    rows: Vec<Vec<i64>>,
}

impl Pascal {
    fn new() -> Self {
        Self {
            rows: vec![vec![1]],
        }
    }

    fn add_row(&mut self) {
        let mut last = self.rows.last().unwrap().clone();
        last.insert(0, 0);
        last.push(0);

        let mut current = Vec::new();
        for w in last.windows(2) {
            current.push(w[0] + w[1]);
        }

        self.rows.push(current);
    }

    fn get_row(&mut self, n: usize) -> Vec<i64> {
        while n >= self.rows.len() {
            self.add_row();
        }

        self.rows[n].clone()
    }

    fn sub_over_vec(&mut self, row: usize, v: &Vec<i64>) -> Vec<i64> {
        let binding = self.get_row(row);
        let r = binding.iter().enumerate().map(|(i, v)| match i % 2 {
            0 => *v,
            1 => -*v,
            _ => unreachable!(),
        });

        let mut ret = Vec::new();

        for x in v.as_slice().windows(r.len()) {
            ret.push(r.clone().rev().zip(x).map(|(a, b)| a * b).sum());
        }

        ret
    }
}

fn read_input(s: &str) -> Result<Vec<Vec<i64>>, ParseError> {
    Ok(s.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect())
}

fn repair_history(h: &Vec<i64>, p: &mut Pascal) -> i64 {
    let result: i64 = *h.last().unwrap();
    let v = p.sub_over_vec(1, h);
    if v.iter().all(|n| *n == 0) {
        result
    } else {
        result + repair_history(&v, p)
    }
}

fn backtrace_history(h: &Vec<i64>, p: &mut Pascal) -> i64 {
    let result: i64 = *h.first().unwrap();
    let v = p.sub_over_vec(1, h);
    if v.iter().all(|n| *n == 0) {
        result
    } else {
        result - backtrace_history(&v, p)
    }
}

fn part_1(input: &str) -> i64 {
    let mut p = Pascal::new();
    read_input(input)
        .unwrap()
        .iter()
        .map(|line| repair_history(line, &mut p))
        .sum()
}

fn part_2(input: &str) -> i64 {
    let mut p = Pascal::new();
    read_input(input)
        .unwrap()
        .iter()
        .map(|line| backtrace_history(line, &mut p))
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

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn check_parse() {
        dbg!(read_input(INPUT));
    }

    #[test]
    fn test_pascal() {
        let mut p = Pascal::new();
        dbg!(p.sub_over_vec(1, &vec![0, 3, 6, 9, 12, 15]));
        dbg!(p.sub_over_vec(2, &vec![0, 3, 6, 9, 12, 15]));
        dbg!(p.sub_over_vec(1, &vec![1, 3, 6, 10, 15, 21]));
        dbg!(p.sub_over_vec(2, &vec![1, 3, 6, 10, 15, 21]));
        dbg!(p.sub_over_vec(3, &vec![1, 3, 6, 10, 15, 21]));
    }

    #[test]
    fn p1() {
        assert_eq!(part_1(INPUT), 114);
    }

    #[test]
    fn p2() {
        assert_eq!(part_2(INPUT), 2);
    }
}
