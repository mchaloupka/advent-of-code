#[derive(Debug)]
struct InputEntry {
    total: i128,
    numbers: Vec<i128>,
}

fn parse_input(input: &str) -> Vec<InputEntry> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(':').expect("Invalid row");

            InputEntry {
                total: left.parse().expect("Expected number for total"),
                numbers: right
                    .split_whitespace()
                    .map(|x| x.parse().expect("Expected only numbers"))
                    .collect(),
            }
        })
        .collect()
}

fn is_valid_1(total: i128, numbers: &Vec<i128>, idx: usize) -> bool {
    let cur_number = numbers[idx];

    if cur_number > total {
        false
    } else if idx == 0 {
        cur_number == total
    } else if total % cur_number == 0 && is_valid_1(total / cur_number, numbers, idx - 1) {
        true
    } else {
        is_valid_1(total - cur_number, numbers, idx - 1)
    }
}

fn part_1(input: &str) {
    let parsed_input = parse_input(input);

    let mut total: i128 = 0;

    for entry in parsed_input {
        if is_valid_1(entry.total, &entry.numbers, entry.numbers.len() - 1) {
            total += entry.total;
        }
    }

    println!("Part 1: {total}");
}

fn is_valid_2(total: i128, numbers: &Vec<i128>, idx: usize) -> bool {
    let cur_number = numbers[idx];

    if cur_number > total {
        false
    } else if idx == 0 {
        cur_number == total
    } else if (total % cur_number == 0 && is_valid_2(total / cur_number, numbers, idx - 1))
        || is_valid_2(total - cur_number, numbers, idx - 1)
    {
        true
    } else {
        let mut remaining_number = cur_number;
        let mut remaining_total = total;

        while remaining_number > 0 {
            let digit = remaining_number % 10;

            if remaining_total >= 0 && remaining_total % 10 == digit {
                remaining_number /= 10;
                remaining_total /= 10;
            } else {
                return false;
            }
        }

        is_valid_2(remaining_total, numbers, idx - 1)
    }
}

fn part_2(input: &str) {
    let parsed_input = parse_input(input);

    let mut total: i128 = 0;

    for entry in parsed_input {
        if is_valid_2(entry.total, &entry.numbers, entry.numbers.len() - 1) {
            total += entry.total;
        }
    }

    println!("Part 1: {total}");
}

pub fn run(input: &str) {
    part_1(input);
    part_2(input);
}
