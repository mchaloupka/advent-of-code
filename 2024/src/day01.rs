use itertools::Itertools;

pub fn run(input: &str) {
    let lines: Vec<(i32, i32)> = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            (
                parts[0].parse::<i32>().expect("Incorrect number"),
                parts[1].parse::<i32>().expect("Incorrect number"),
            )
        })
        .collect();

    let left_numbers = lines.iter().map(|l| l.0).sorted().collect::<Vec<_>>();

    let right_numbers = lines.iter().map(|l| l.1).sorted().collect::<Vec<_>>();

    let mut difference = 0;

    for i in 0..left_numbers.len() {
        difference += (left_numbers[i] - right_numbers[i]).abs();
    }

    println!("Output Part 1: {difference}");

    let mut difference_b = 0;
    let mut right_pos = 0;

    for left_number in left_numbers {
        let mut right_count = 0;

        while right_pos < right_numbers.len() && right_numbers[right_pos] < left_number {
            right_pos += 1;
        }

        while right_pos < right_numbers.len() && right_numbers[right_pos] == left_number {
            right_pos += 1;
            right_count += 1;
        }

        difference_b += left_number * right_count;
    }

    println!("Output Part 2: {difference_b}");
}
