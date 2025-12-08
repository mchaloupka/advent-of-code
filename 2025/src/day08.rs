use std::collections::{HashMap, HashSet};

type Point = [f64; 3];

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let coords: Vec<f64> = line.split(',').map(|num| num.parse().unwrap()).collect();
            [coords[0], coords[1], coords[2]]
        })
        .collect()
}

fn point_distance(a: &Point, b: &Point) -> f64 {
    ((a[0] - b[0]) * (a[0] - b[0]) + (a[1] - b[1]) * (a[1] - b[1]) + (a[2] - b[2]) * (a[2] - b[2]))
        .sqrt()
}

pub fn run(_input: &str) {
    let points = parse_input(_input);
    let mut connection_groups: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut point_to_group: HashMap<usize, usize> = HashMap::new();

    for (i, _) in points.iter().enumerate() {
        let mut connection_group = HashSet::new();
        connection_group.insert(i);
        connection_groups.insert(i, connection_group);
        point_to_group.insert(i, i);
    }

    let mut distance_pairs = points
        .iter()
        .enumerate()
        .flat_map(|(i, point)| {
            points
                .iter()
                .enumerate()
                .skip(i + 1)
                .map(move |(o, other_point)| {
                    let distance = point_distance(point, other_point);
                    ((i, o), distance)
                })
        })
        .collect::<Vec<_>>();

    distance_pairs.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    for (iter, ((point_a_id, point_b_id), _)) in distance_pairs.iter().enumerate() {
        let group_a_id = *point_to_group.get(point_a_id).unwrap();
        let group_b_id = *point_to_group.get(point_b_id).unwrap();

        if group_a_id != group_b_id {
            let group_b = connection_groups.get(&group_b_id).unwrap().clone();
            connection_groups.remove(&group_b_id);
            let group_a = connection_groups.get_mut(&group_a_id).unwrap();

            for pb in group_b {
                group_a.insert(pb);
                point_to_group.insert(pb, group_a_id);
            }
        }

        if iter == 999 {
            let mut groups_by_size: Vec<_> = connection_groups.values().collect();
            groups_by_size.sort_by_key(|group| -(i32::try_from(group.len()).unwrap()));
            let part_1 = groups_by_size
                .iter()
                .take(3)
                .map(|g| g.len())
                .product::<usize>();
            println!("Part 1: {}", part_1);
        }

        if connection_groups.len() == 1 {
            let point_a = points[*point_a_id];
            let point_b = points[*point_b_id];
            let part_2 = point_a[0] * point_b[0];
            println!("Part 2: {}", part_2);
            break;
        }
    }
}
