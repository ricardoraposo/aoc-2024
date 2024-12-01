use std::fs;

pub struct Rows {
    first: Vec<u32>,
    second: Vec<u32>,
}

pub fn run() {
    let input = fs::read_to_string("./inputs/day_one.txt").unwrap();

    let rows = get_rows(&input);

    part1(&rows);
    part2(&rows);
}

fn part1(rows: &Rows) {
    let mut sum: u32 = 0;
    for (i, value) in rows.first.iter().enumerate() {
        let second_value = rows.second[i];
        if value >= &second_value {
            sum += value - second_value;
        } else {
            sum += second_value - value;
        }
    }

    println!("Day 1, part 1: {sum}")
}

fn part2(rows: &Rows) {
    let mut res = 0;
    for first in &rows.first {
        let mut repetitions = 0;
        for second in &rows.second {
            if second == first {
                repetitions += 1;
            }
        }
        res += first * repetitions;
    }

    println!("Day 2, part 2: {res}")
}

fn get_rows(input: &str) -> Rows {
    let lines = input.split("\n");

    let mut first_row: Vec<u32> = Vec::new();
    let mut second_row: Vec<u32> = Vec::new();

    for line in lines {
        let pair: Vec<&str> = line.split_whitespace().collect();

        if pair.len() == 2 {
            first_row.push(pair[0].parse::<u32>().unwrap());
            second_row.push(pair[1].parse::<u32>().unwrap());
        }
    }

    first_row.sort();
    second_row.sort();

    Rows {
        first: first_row,
        second: second_row,
    }
}
