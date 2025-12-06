fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    let mut entries: Vec<usize> = Vec::new();

    let mut is_parsing_ranges = true;

    for line in input.lines() {
        if is_parsing_ranges {
            if line.is_empty() {
                is_parsing_ranges = false;
            } else {
                let splitted: Vec<usize> = line
                    .split("-")
                    .map(|x| x.parse().expect("Has to be number"))
                    .collect();

                ranges.push((splitted[0], splitted[1]));
            }
        } else {
            entries.push(line.parse().expect("Has to be number"));
        }
    }

    (ranges, entries)
}

pub fn run(input: &str) {
    let (mut ranges, entries) = parse_input(input);

    let part_1 = entries
        .iter()
        .filter(|entry| {
            for range in &ranges {
                if **entry >= range.0 && **entry <= range.1 {
                    return true;
                }
            }

            false
        })
        .count();

    println!("Part 1: {}", part_1);

    let mut active_ranges: Vec<(usize, usize)> = Vec::new();

    let mut last_start = 0;
    let mut part_2 = 0;

    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    ranges.iter().for_each(|range| {
        if active_ranges.is_empty() {
            last_start = range.0;
        } else {
            while let Some(active_range) = active_ranges.last() {
                if active_range.1 < range.0 {
                    if last_start <= active_range.1 {
                        part_2 += active_range.1 - last_start + 1;
                        last_start = active_range.1 + 1;
                    }

                    active_ranges.pop();
                } else {
                    break;
                }
            }

            if active_ranges.is_empty() {
                last_start = range.0;
            }
        }

        active_ranges.push(*range);
        active_ranges.sort_by(|a, b| a.1.cmp(&b.1));
    });

    if let Some(final_range) = active_ranges.last() {
        part_2 += final_range.1 - last_start + 1;
    }

    println!("Part 2: {}", part_2);
}
