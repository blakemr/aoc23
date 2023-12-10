use std::{fs::File, io::Read};

fn part_1(input: &str) -> u32 {
    todo!()
}

fn part_2(input: &str) -> u32 {
    todo!()
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

    const INPUT: &str = "";

    #[test]
    fn p1() {
        todo!()
    }

    #[test]
    fn p2() {
        todo!()
    }
}
