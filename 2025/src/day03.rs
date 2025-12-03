fn get_largest_number(batteries: &[u32], num_batteries: usize) -> u64 {
    let mut number: u64 = 0;
    let mut largest_idx = 0;

    for remaining_batteries in (0..num_batteries).rev() {
        for i in largest_idx + 1..batteries.len() - remaining_batteries {
            let cur_number = batteries[i];
            if cur_number > batteries[largest_idx] {
                largest_idx = i;
            }
            if cur_number == 9 {
                break;
            }
        }

        number *= 10;
        number += <u32 as Into<u64>>::into(batteries[largest_idx]);

        largest_idx += 1;
    }

    number
}

pub fn run(input: &str) {
    let mut part_1 = 0;
    let mut part_2 = 0;

    input.lines().for_each(|line| {
        let numbers: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();

        part_1 += get_largest_number(&numbers, 2);
        part_2 += get_largest_number(&numbers, 12);
    });

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
