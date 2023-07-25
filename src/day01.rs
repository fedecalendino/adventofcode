use std::fs::read_to_string;

fn read(name: &str) -> Vec<Option<i32>> {
    let path = format!("./files/{name}.txt", name = name);

    let mut values = Vec::new();

    for line in read_to_string(path).unwrap().lines() {
        // add options to support empty lines
        let value: Option<i32> = match line.parse() {
            Ok(value) => Some(value),
            _ => None,
        };

        values.push(value)
    }

    values
}

fn part01(input: &str) -> i32 {
    let values = read(input);

    let mut elf: i32 = 0;
    let mut max: i32 = 0;

    for each in values.iter() {
        match each {
            Some(value) => elf += value,
            None => {
                if elf > max {
                    max = elf;
                }

                elf = 0;
            }
        }
    }

    max
}

fn part02(input: &str) -> i32 {
    let values = read(input);

    let mut elves = Vec::new();
    let mut elf = 0;

    for each in values.iter() {
        match each {
            Some(value) => elf += value,
            None => {
                elves.push(elf);
                elf = 0;
            }
        }
    }

    elves.push(elf);
    elves.sort();

    let n = elves.len();
    elves[(n - 3)..].iter().sum()
}

pub fn solve() {
    // https://adventofcode.com/2022/day/1

    assert_eq!(part01("day01/test01"), 24000); // ⭐
    println!("Day 01 - Part 01 = {}", part01("day01/input"));

    assert_eq!(part02("day01/test01"), 45000); // ⭐
    println!("Day 01 - Part 02 = {}", part02("day01/input"));
}
