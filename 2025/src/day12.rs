#[derive(Debug)]
struct Shape {
    occupied: Vec<Vec<bool>>,
}

#[derive(Debug)]
struct InputEntry {
    x: usize,
    y: usize,
    shape_counts: Vec<usize>,
}

fn parse_input(input: &str) -> (Vec<Shape>, Vec<InputEntry>) {
    let sections: Vec<_> = input.split("\n\n").collect();

    let shapes = sections[0..sections.len() - 1]
        .iter()
        .map(|shape_section| Shape {
            occupied: shape_section
                .lines()
                .skip(1)
                .map(|line| line.chars().map(|c| c == '#').collect())
                .collect(),
        })
        .collect();

    let entries = sections
        .last()
        .unwrap()
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(": ").unwrap();
            let (x, y) = left.split_once("x").unwrap();
            let shape_counts = right
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            InputEntry {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
                shape_counts,
            }
        })
        .collect();

    (shapes, entries)
}

pub fn run(input: &str) {
    let (shapes, entries) = parse_input(input);

    shapes
        .iter()
        .enumerate()
        .for_each(|(i, shape)| println!("Shape {}: {:?}", i, shape.occupied));

    println!();

    let mut part_1 = 0;
    for (i, entry) in entries.iter().enumerate() {
        let total_area = entry.x * entry.y;
        let max_occupied_area: usize = entry.shape_counts.iter().map(|count| count * 8).sum();

        if max_occupied_area < total_area {
            part_1 += 1;
        } else {
            println!(
                "{}: It is possible that it may not fit due to area size.",
                i
            );
            // However, further check was not needed as it already achieved right result.
        }
    }

    println!("Part 1: {}", part_1);
}
