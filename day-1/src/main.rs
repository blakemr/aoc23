use std::{fs::File, io::Read};

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
        let first = line.chars().filter_map(|c| c.to_digit(10)).next().unwrap();

        let last = line
            .chars()
            .filter_map(|c| c.to_digit(10))
            .next_back()
            .unwrap();
        sum += (first * 10) + last;
    }

    sum
}

fn part_2(input: &str) -> u32 {
    let mut sum = 0;
    let keywords = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five",
        "six", "seven", "eight", "nine",
    ];

    for mut line in input.lines() {
        let mut first = keywords.iter().find(|first| line.starts_with(*first));

        while first.is_none() && !line.is_empty() {
            line = &line[1..];
            first = keywords.iter().find(|first| line.starts_with(*first));
        }

        let first = first
            .unwrap()
            .replace("one", "1")
            .replace("two", "2")
            .replace("three", "3")
            .replace("four", "4")
            .replace("five", "5")
            .replace("six", "6")
            .replace("seven", "7")
            .replace("eight", "8")
            .replace("nine", "9")
            .chars()
            .next()
            .unwrap()
            .to_digit(10)
            .unwrap();

        let mut last = keywords.iter().find(|last| line.ends_with(*last));

        while last.is_none() && !line.is_empty() {
            line = &line[..line.len() - 1];
            last = keywords.iter().find(|last| line.ends_with(*last));
        }

        let last = last
            .unwrap()
            .replace("one", "1")
            .replace("two", "2")
            .replace("three", "3")
            .replace("four", "4")
            .replace("five", "5")
            .replace("six", "6")
            .replace("seven", "7")
            .replace("eight", "8")
            .replace("nine", "9")
            .chars()
            .next()
            .unwrap()
            .to_digit(10)
            .unwrap();

        sum += (first * 10) + last;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let test_input = "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet";

        assert_eq!(part_1(test_input), 142);
    }

    #[test]
    fn p2() {
        let test_input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

        assert_eq!(part_2(test_input), 281);
    }
}
