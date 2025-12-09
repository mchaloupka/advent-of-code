use std::collections::{HashSet, VecDeque};

type Point = (i64, i64);

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts
                .next()
                .and_then(|s| s.trim().parse::<_>().ok())
                .unwrap();
            let y = parts
                .next()
                .and_then(|s| s.trim().parse::<_>().ok())
                .unwrap();
            (x, y)
        })
        .collect()
}

pub fn run(input: &str) {
    let red_points = parse_input(input);

    let mut part_1 = 0;

    for (i, point) in red_points.iter().enumerate() {
        for other_point in red_points.iter().skip(i + 1) {
            let area =
                ((point.0 - other_point.0).abs() + 1) * ((point.1 - other_point.1).abs() + 1);
            if area > part_1 {
                part_1 = area;
            }
        }
    }

    println!("Part 1: {}", part_1);

    let mut inside_points: HashSet<Point> = HashSet::new();
    let min_point = (
        red_points.iter().map(|p| p.0).min().unwrap(),
        red_points.iter().map(|p| p.1).min().unwrap(),
    );
    let max_point = (
        red_points.iter().map(|p| p.0).max().unwrap(),
        red_points.iter().map(|p| p.1).max().unwrap(),
    );

    for i in 0..red_points.len() {
        let p1 = if i == 0 {
            red_points[red_points.len() - 1]
        } else {
            red_points[i - 1]
        };
        let p2 = red_points[i];
        inside_points.insert(p1);
        inside_points.insert(p2);
        let diff_x = if p2.0 - p1.0 != 0 {
            (p2.0 - p1.0) / (p2.0 - p1.0).abs()
        } else {
            0
        };
        let diff_y = if p2.1 - p1.1 != 0 {
            (p2.1 - p1.1) / (p2.1 - p1.1).abs()
        } else {
            0
        };

        let mut current_point = p1;
        while current_point != p2 {
            inside_points.insert(current_point.clone());
            current_point = (current_point.0 + diff_x, current_point.1 + diff_y);
        }
    }

    let mut walk_through: VecDeque<Point> = VecDeque::new();
    walk_through.push_back((min_point.0 - 1, min_point.1 - 1));
    let mut outside_points: HashSet<Point> = HashSet::new();

    while let Some(current_point) = walk_through.pop_front() {
        if outside_points.contains(&current_point) {
            continue;
        }

        outside_points.insert(current_point);

        let neighbors = [
            (current_point.0 - 1, current_point.1),
            (current_point.0, current_point.1 - 1),
            (current_point.0 + 1, current_point.1),
            (current_point.0, current_point.1 + 1),
        ];

        for neighbor in neighbors {
            if neighbor.0 < min_point.0 - 1
                || neighbor.0 > max_point.0 + 1
                || neighbor.1 < min_point.1 - 1
                || neighbor.1 > max_point.1 + 1
            {
                continue;
            }

            if inside_points.contains(&neighbor) {
                continue;
            }

            if outside_points.contains(&neighbor) {
                continue;
            }

            walk_through.push_front(neighbor);
        }
    }

    let mut part_2 = 0;

    for (i, point) in red_points.iter().enumerate() {
        for other_point in red_points.iter().skip(i + 1) {
            let min_x = point.0.min(other_point.0);
            let max_x = point.0.max(other_point.0);
            let min_y = point.1.min(other_point.1);
            let max_y = point.1.max(other_point.1);

            let mut is_enclosed = true;

            for x in min_x..=max_x {
                if outside_points.contains(&(x, min_y)) || outside_points.contains(&(x, max_y)) {
                    is_enclosed = false;
                    break;
                }
            }

            if is_enclosed {
                for y in min_y..=max_y {
                    if outside_points.contains(&(min_x, y)) || outside_points.contains(&(max_x, y))
                    {
                        is_enclosed = false;
                        break;
                    }
                }
            }

            if !is_enclosed {
                continue;
            }

            let area =
                ((point.0 - other_point.0).abs() + 1) * ((point.1 - other_point.1).abs() + 1);

            if area > part_2 {
                part_2 = area;
            }
        }
    }

    println!("Part 2: {}", part_2);
}
