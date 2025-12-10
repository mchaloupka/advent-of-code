use geo::{Contains, Polygon};

type Point = (f64, f64);

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

    let mut part_1 = 0.0;

    let mut areas_by_size: Vec<_> = red_points
        .iter()
        .enumerate()
        .flat_map(|(i, point)| {
            red_points
                .iter()
                .skip(i + 1)
                .map(|other_point| {
                    let area = ((point.0 - other_point.0).abs() + 1.0)
                        * ((point.1 - other_point.1).abs() + 1.0);

                    (area, (point, other_point))
                })
                .collect::<Vec<_>>()
        })
        .collect();
    areas_by_size.sort_by(|x, y| y.0.partial_cmp(&x.0).unwrap());

    for (i, point) in red_points.iter().enumerate() {
        for other_point in red_points.iter().skip(i + 1) {
            let area =
                ((point.0 - other_point.0).abs() + 1.0) * ((point.1 - other_point.1).abs() + 1.0);
            if area > part_1 {
                part_1 = area;
            }
        }
    }

    println!("Part 1: {}", areas_by_size[0].0);

    let mut line_string_points = red_points.clone();
    line_string_points.push(*red_points.first().unwrap());

    let polygon: Polygon<_> = Polygon::new(geo::LineString::from(line_string_points), vec![]);

    for (size, (point, other_point)) in areas_by_size {
        let area_polygon = Polygon::new(
            geo::LineString::from(vec![
                *point,
                (point.0, other_point.1),
                *other_point,
                (other_point.0, point.1),
                *point,
            ]),
            vec![],
        );

        if polygon.contains(&area_polygon) {
            println!("Part 2: {}", size);
            break;
        }
    }
}
