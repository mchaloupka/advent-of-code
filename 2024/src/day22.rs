use std::collections::{HashMap, HashSet};

fn prune(num: usize) -> usize {
    num & 0xffffff
}

fn next_secret_number(num: usize) -> usize {
    let step_1 = prune(num ^ (num << 6));
    let step_2 = prune(step_1 ^ (step_1 >> 5));
    prune(step_2 ^ (step_2 << 11))
}

fn n_th_number(num: usize, iter: usize) -> usize {
    let mut output = num;
    for _ in 0..iter {
        output = next_secret_number(output);
    }
    output
}

fn part_1(input: &str) {
    let mut total = 0;

    for line in input.lines() {
        let number: usize = line.parse().unwrap();
        total += n_th_number(number, 2000);
    }

    println!("Part 1: {}", total);
}

fn sequence_of_prices(init_num: usize, iter: usize) -> Vec<i32> {
    let mut output = Vec::new();
    let mut secret_number = init_num;

    for _ in 0..iter {
        output.push((secret_number % 10) as i32);
        secret_number = next_secret_number(secret_number);
    }

    output
}

fn sequence_to_identifier(sequence: &(i32, i32, i32, i32)) -> i32 {
    (sequence.3 + 10)
        + (sequence.2 + 10) * 100
        + (sequence.1 + 10) * 10000
        + (sequence.0 + 10) * 1000000
}

fn get_bananas_per_sequence(prices: &[i32]) -> HashMap<i32, i32> {
    let mut output = HashMap::new();

    for (i, &price) in prices.iter().enumerate().skip(4) {
        let diff_0 = prices[i - 3] - prices[i - 4];
        let diff_1 = prices[i - 2] - prices[i - 3];
        let diff_2 = prices[i - 1] - prices[i - 2];
        let diff_3 = prices[i] - prices[i - 1];
        let sequence = (diff_0, diff_1, diff_2, diff_3);
        output
            .entry(sequence_to_identifier(&sequence))
            .or_insert(price);
    }

    output
}

fn part_2(input: &str) {
    let bananas_per_buyer_and_sequence: Vec<_> = input
        .lines()
        .map(|line| {
            let init_num = line.parse().unwrap();
            let prices = sequence_of_prices(init_num, 2000);
            get_bananas_per_sequence(&prices)
        })
        .collect();

    let all_sequences: HashSet<_> = bananas_per_buyer_and_sequence
        .iter()
        .flat_map(|map| map.keys())
        .collect();

    let mut best_bananas = None;

    for sequence in all_sequences {
        let mut total_bananas: i32 = 0;

        for buyer_map in bananas_per_buyer_and_sequence.iter() {
            total_bananas += buyer_map.get(sequence).unwrap_or(&0);
        }

        match best_bananas {
            Some(x) if x < total_bananas => {
                best_bananas = Some(total_bananas);
            }
            None => {
                best_bananas = Some(total_bananas);
            }
            _ => {}
        }
    }

    println!("Part 2: {}", best_bananas.unwrap());
}

pub fn run(input: &str) {
    part_1(input);
    part_2(input);
}
