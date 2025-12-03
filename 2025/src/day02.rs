pub fn run(input: &str) {
    let ranges = input.trim().split(",").map(|range| {
        let parts: Vec<&str> = range.split("-").collect();
        (
            parts[0].parse::<i64>().expect("Incorrect number"),
            parts[1].parse::<i64>().expect("Incorrect number"),
        )
    });

    let mut part_1 = 0;
    let mut part_2 = 0;

    for (start, end) in ranges {
        for i in start..=end {
            let digits_count = i.to_string().len() as i64;

            for digits_block in 1..=digits_count / 2 {
                if digits_count % digits_block != 0 {
                    continue;
                }

                let right_part = i % 10i64.pow(digits_block as u32);
                let mut all_match = true;

                for left_offset in 1..digits_count / digits_block {
                    let without_right = i / 10i64.pow((digits_block * left_offset) as u32);
                    let left_part = without_right % 10i64.pow(digits_block as u32);

                    if left_part != right_part {
                        all_match = false;
                        break;
                    }
                }

                if all_match {
                    part_2 += i;
                    break;
                }
            }

            // odd numbers are always valid, so we look only into even ones
            if digits_count % 2 == 0 {
                let offset = digits_count / 2;
                let left_part = i / 10i64.pow(offset as u32);
                let right_part = i % 10i64.pow(offset as u32);
                if left_part == right_part {
                    part_1 += i;
                }
            }
        }
    }

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
