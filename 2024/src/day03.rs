use regex::Regex;

fn part_1(input: &str) {
    let pattern = r"mul\((\d+),(\d+)\)";
    let re = Regex::new(pattern).unwrap();

    let mut sum = 0;

    for cap in re.captures_iter(input) {
        let a: i32 = cap[1].parse().expect("Has to be number");
        let b: i32 = cap[2].parse().expect("Has to be number");
        sum += a * b;
    }

    println!("Part 1: {sum}");
}

fn part_2(input: &str) {
    let pattern = r"mul\((\d+),(\d+)\)";
    let re = Regex::new(pattern).unwrap();

    let mut sum = 0;

    let mut start_index: usize = 0;
    let mut end_index = input.find("don't()").unwrap_or(input.len() - 1);

    loop {
        let input_slice = &input[start_index..end_index];

        for cap in re.captures_iter(input_slice) {
            let a: i32 = cap[1].parse().expect("Has to be number");
            let b: i32 = cap[2].parse().expect("Has to be number");
            sum += a * b;
        }

        start_index = end_index + "don't()".len();

        if start_index < input.len() {
            match input[start_index..].find("do()") {
                Some(idx) => {
                    start_index += idx + "do()".len();
                    let remaining = &input[start_index..];
                    end_index =
                        start_index + remaining.find("don't()").unwrap_or(remaining.len() - 1);
                }
                None => start_index = input.len(),
            }
        }

        if start_index >= input.len() {
            break;
        }
    }

    println!("Part 2: {sum}");
}

pub fn run(input: &str) {
    part_1(input);
    part_2(input);
}
