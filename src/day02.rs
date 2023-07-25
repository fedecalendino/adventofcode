use std::fs::read_to_string;

struct Play {
    me: char,
    opponent: char,
}

fn read(name: &str) -> Vec<Play> {
    let path = format!("./files/{name}.txt", name = name);

    let mut values = Vec::new();

    for line in read_to_string(path).unwrap().lines() {
        let play = Play {
            me: line.chars().nth(2).unwrap(),
            opponent: line.chars().nth(0).unwrap(),
        };

        values.push(play)
    }

    values
}

fn round_score(play: Play) -> i32 {
    match play.opponent {
        'A' => match play.me {
            'X' => 3 + 1,
            'Y' => 6 + 2,
            'Z' => 0 + 3,
            _ => panic!(),
        },
        'B' => match play.me {
            'X' => 0 + 1,
            'Y' => 3 + 2,
            'Z' => 6 + 3,
            _ => panic!(),
        },
        'C' => match play.me {
            'X' => 6 + 1,
            'Y' => 0 + 2,
            'Z' => 3 + 3,
            _ => panic!(),
        },
        _ => panic!("Invalid move"),
    }
}

fn find_new_play(play: Play) -> char {
    match play.opponent {
        'A' => match play.me {
            'X' => 'Z',
            'Y' => 'X',
            'Z' => 'Y',
            _ => panic!(),
        },
        'B' => match play.me {
            'X' => 'X',
            'Y' => 'Y',
            'Z' => 'Z',
            _ => panic!(),
        },
        'C' => match play.me {
            'X' => 'Y',
            'Y' => 'Z',
            'Z' => 'X',
            _ => panic!(),
        },
        _ => panic!("Invalid move"),
    }
}

fn part01(input: &str) -> i32 {
    let plays = read(input);

    let mut total = 0;

    for play in plays {
        total += round_score(play);
    }

    total
}

fn part02(input: &str) -> i32 {
    let plays = read(input);

    let mut total = 0;

    for play in plays {
        let new_play = Play {
            opponent: play.opponent,
            me: find_new_play(play),
        };

        total += round_score(new_play);
    }

    total
}

pub fn solve() {
    // https://adventofcode.com/2022/day/2

    assert_eq!(part01("day02/test01"), 15); // ⭐
    println!("Day 02 - Part 01 = {}", part01("day02/input"));

    assert_eq!(part02("day02/test01"), 12); // ⭐
    println!("Day 02 - Part 02 = {}", part02("day02/input"));
}
