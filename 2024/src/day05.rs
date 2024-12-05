use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut entries: Vec<Vec<i32>> = Vec::new();

    let mut is_parsing_rules = true;

    for line in input.lines() {
        if is_parsing_rules {
            if line.is_empty() {
                is_parsing_rules = false;
            } else {
                let splitted: Vec<i32> = line
                    .split("|")
                    .map(|x| x.parse().expect("Has to be number"))
                    .collect();

                rules.push((splitted[0], splitted[1]));
            }
        } else {
            entries.push(
                line.split(",")
                    .map(|x| x.parse().expect("Has to be number"))
                    .collect(),
            );
        }
    }

    (rules, entries)
}

fn is_valid(forbidden_map: &HashMap<i32, Vec<i32>>, entry: &Vec<i32>) -> bool {
    let mut not_allowed_numbers: HashSet<i32> = HashSet::new();
    let mut is_valid = true;

    for num in entry {
        if not_allowed_numbers.contains(num) {
            is_valid = false;
            break;
        }

        if let Some(to_forbid) = forbidden_map.get(num) {
            for next in to_forbid {
                not_allowed_numbers.insert(*next);
            }
        }
    }

    is_valid
}

fn create_forbidden_map(rules: &Vec<(i32, i32)>) -> HashMap<i32, Vec<i32>> {
    let mut forbidden_map: HashMap<i32, Vec<i32>> = HashMap::new();

    for rule in rules {
        forbidden_map.entry(rule.1).or_default().push(rule.0);
    }

    forbidden_map
}

fn part_1(input: &str) {
    let (rules, entries) = parse_input(input);
    let forbidden_map = create_forbidden_map(&rules);

    let mut total_count = 0;

    for entry in entries {
        if is_valid(&forbidden_map, &entry) {
            total_count += entry[entry.len() / 2];
        }
    }

    println!("Part 1: {total_count}");
}

fn part_2(input: &str) {
    let (rules, entries) = parse_input(input);
    let forbidden_map = create_forbidden_map(&rules);

    let mut total_count = 0;

    for entry in entries {
        if !is_valid(&forbidden_map, &entry) {
            let mut applicable_rules: Vec<&(i32, i32)> = rules
                .iter()
                .filter(|x| entry.contains(&x.0) && entry.contains(&x.1))
                .collect();

            let mut final_entry: Vec<i32> = Vec::new();

            while !applicable_rules.is_empty() {
                let maybe_next_number = applicable_rules
                    .iter()
                    .map(|x| x.0)
                    .find(|x| applicable_rules.iter().all(|y| y.1 != *x));

                match maybe_next_number {
                    Some(next_number) => {
                        final_entry.push(next_number);
                        applicable_rules.retain(|x| x.0 != next_number && x.1 != next_number);
                    }
                    None => panic!("No possible next number"),
                }
            }

            let remaining_numbers: Vec<_> = entry
                .into_iter()
                .filter(|x| !final_entry.contains(x))
                .collect();

            for x in remaining_numbers {
                final_entry.push(x);
            }

            total_count += final_entry[final_entry.len() / 2];
        }
    }

    println!("Part 2: {total_count}");
}

pub fn run(input: &str) {
    part_1(input);
    part_2(input);
}
