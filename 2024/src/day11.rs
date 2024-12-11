use std::collections::{HashMap, LinkedList};

fn parse_input(input: &str) -> Vec<i64> {
    input
        .split_whitespace()
        .map(|x| x.parse().expect("Has to be number"))
        .collect()
}

fn count_digits(mut n: i64) -> u32 {
    if n == 0 {
        return 1; // Special case: 0 has one digit.
    }

    let mut count = 0;

    while n > 0 {
        n /= 10;
        count += 1;
    }

    count
}

fn part_1(input: &str) {
    let mut stones = parse_input(input);

    for _ in 0..25 {
        let mut after_blink_stones = Vec::new();

        for stone in stones {
            if stone == 0 {
                after_blink_stones.push(1);
            } else {
                let digits = count_digits(stone);

                if digits % 2 == 0 {
                    let divisor = 10_i64.pow(digits / 2);
                    after_blink_stones.push(stone / divisor);
                    after_blink_stones.push(stone % divisor);
                } else {
                    after_blink_stones.push(stone * 2024);
                }
            }
        }

        stones = after_blink_stones;
    }

    println!("Part 1: {}", stones.len());
}

fn part_2(input: &str) {
    let stones = parse_input(input);

    let mut cache: HashMap<(i64, u32), u64> = HashMap::new();
    let mut to_process: LinkedList<(i64, u32)> = LinkedList::new();

    for stone in &stones {
        to_process.push_front((*stone, 75));
    }

    while let Some((stone, blinks)) = to_process.pop_front() {
        if cache.contains_key(&(stone, blinks)) {
            continue;
        }

        if blinks == 0 {
            cache.insert((stone, 0), 1);
        } else if stone == 0 {
            match cache.get(&(1, blinks - 1)) {
                Some(x) => {
                    cache.insert((stone, blinks), *x);
                }
                None => {
                    to_process.push_front((stone, blinks));
                    to_process.push_front((1, blinks - 1));
                }
            }
        } else {
            let digits = count_digits(stone);

            if digits % 2 == 0 {
                let divisor = 10_i64.pow(digits / 2);
                let left_stone = stone / divisor;
                let right_stone = stone % divisor;

                match (
                    cache.get(&(left_stone, blinks - 1)),
                    cache.get(&(right_stone, blinks - 1)),
                ) {
                    (None, None) => {
                        to_process.push_front((stone, blinks));
                        to_process.push_front((left_stone, blinks - 1));
                        to_process.push_front((right_stone, blinks - 1));
                    }
                    (None, Some(_)) => {
                        to_process.push_front((stone, blinks));
                        to_process.push_front((left_stone, blinks - 1));
                    }
                    (Some(_), None) => {
                        to_process.push_front((stone, blinks));
                        to_process.push_front((right_stone, blinks - 1));
                    }
                    (Some(x), Some(y)) => {
                        cache.insert((stone, blinks), x + y);
                    }
                }
            } else {
                let next_stone = stone * 2024;
                match cache.get(&(next_stone, blinks - 1)) {
                    Some(x) => {
                        cache.insert((stone, blinks), *x);
                    }
                    None => {
                        to_process.push_front((stone, blinks));
                        to_process.push_front((next_stone, blinks - 1));
                    }
                }
            }
        }
    }

    let mut stone_count = 0;
    for stone in stones {
        stone_count += cache.get(&(stone, 75)).expect("Missing value in cache");
    }

    println!("Part 2: {stone_count}");
}

pub fn run(input: &str) {
    part_1(input);
    part_2(input);
}
