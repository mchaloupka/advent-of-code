pub fn run(input: &str) {
    let numbers = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse::<i32>().expect("Fail to parse"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut safe_count = 0;

    for entry in &numbers {
        let is_increasing = entry[0] < entry[1];
        let mut is_safe = true;

        for i in 1..entry.len() {
            let diff = entry[i] - entry[i - 1];

            if diff.abs() < 1
                || diff.abs() > 3
                || (is_increasing && diff < 0)
                || (!is_increasing && diff > 0)
            {
                is_safe = false;
                break;
            }
        }

        if is_safe {
            safe_count += 1;
        }
    }

    println!("Part 1: {safe_count}");

    safe_count = 0;

    for entry in &numbers {
        let is_increasing = entry[0] < entry[1];
        let mut is_safe = true;

        for i in 1..entry.len() {
            let diff = entry[i] - entry[i - 1];

            if diff.abs() < 1
                || diff.abs() > 3
                || (is_increasing && diff < 0)
                || (!is_increasing && diff > 0)
            {
                is_safe = false;
                break;
            }
        }

        if !is_safe {
            for skip_index in 0..entry.len() {
                is_safe = true;
                let is_increasing = match skip_index {
                    0 => entry[1] < entry[2],
                    1 => entry[0] < entry[2],
                    _ => entry[0] < entry[1],
                };

                for i in 1..entry.len() {
                    if i == skip_index {
                        continue;
                    }
                    let previous = if (i - 1) == skip_index {
                        if i < 2 {
                            continue;
                        }
                        i - 2
                    } else {
                        i - 1
                    };

                    let diff = entry[i] - entry[previous];

                    if diff.abs() < 1
                        || diff.abs() > 3
                        || (is_increasing && diff < 0)
                        || (!is_increasing && diff > 0)
                    {
                        is_safe = false;
                        break;
                    }
                }

                if is_safe {
                    break;
                }
            }
        }

        if is_safe {
            safe_count += 1;
        }
    }

    println!("Part 2: {safe_count}");
}
