use cached::proc_macro::cached;
use cached::SizedCache;
use std::collections::HashMap;

#[derive(Debug)]
struct Machine<'a> {
    transitions: HashMap<&'a str, Vec<&'a str>>,
}

fn parse_input<'a>(input: &'a str) -> Machine<'a> {
    let mut transitions: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let (from_raw, to_list) = line.split_once(":").unwrap();
        let from = from_raw.trim();

        to_list.split_whitespace().for_each(|to| {
            transitions.entry(from).or_default().push(to);
        });
    }
    Machine { transitions }
}

#[cached(
    ty = "SizedCache<String, usize>",
    create = "{ SizedCache::with_size(1000000) }",
    convert = r#"{ format!("{}{}", from, to) }"#
)]
fn find_path_count(machine: &Machine, from: &str, to: &str) -> usize {
    if from == to {
        1
    } else {
        machine
            .transitions
            .get(from)
            .unwrap_or(&Vec::new())
            .iter()
            .map(|next| find_path_count(machine, next, to))
            .sum()
    }
}

pub fn run(input: &str) {
    let machine = parse_input(input);

    let part_1 = find_path_count(&machine, "you", "out");

    println!("Part 1: {}", part_1);

    let possible_paths = [["svr", "dac", "fft", "out"], ["svr", "fft", "dac", "out"]];

    let part_2: usize = possible_paths
        .iter()
        .map(|path| {
            path.windows(2)
                .map(|window| find_path_count(&machine, window[0], window[1]))
                .product::<usize>()
        })
        .sum();

    println!("Part 2: {}", part_2);
}
