fn check_xmas(
    arr: &Vec<Vec<char>>,
    startx: usize,
    starty: usize,
    x: i32,
    y: i32,
    letters: &[char],
) -> bool {
    if arr[startx][starty] == letters[0] {
        if letters.len() == 1 {
            true
        } else if (x < 0 && startx == 0)
            || (x > 0 && startx == arr.len() - 1)
            || (y < 0 && starty == 0)
            || (y > 0 && starty == arr[startx].len() - 1)
        {
            false
        } else {
            check_xmas(
                arr,
                ((startx as i32) + x) as usize,
                ((starty as i32) + y) as usize,
                x,
                y,
                &letters[1..],
            )
        }
    } else {
        false
    }
}

fn part_1(input: &str) {
    let arr: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut count = 0;

    let word = ['X', 'M', 'A', 'S'];

    for x in 0..arr.len() {
        for y in 0..arr[x].len() {
            for ox in [-1, 0, 1] {
                for oy in [-1, 0, 1] {
                    if check_xmas(&arr, x, y, ox, oy, &word) {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("Part 1: {count}");
}

fn part_2(input: &str) {
    let arr: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut count = 0;

    let directions = [
        [(-1, -1), (1, -1), (-1, 1), (1, 1)],
        [(1, -1), (1, 1), (-1, 1), (-1, -1)],
        [(1, 1), (-1, 1), (-1, -1), (1, -1)],
        [(-1, 1), (-1, -1), (1, -1), (1, 1)],
    ];

    let words = [['A', 'M'], ['A', 'M'], ['A', 'S'], ['A', 'S']];

    for x in 0..arr.len() {
        for y in 0..arr[x].len() {
            for variant in directions {
                let mut all_match = true;

                for i in 0..variant.len() {
                    let direction = variant[i];
                    let word = words[i];

                    if !check_xmas(&arr, x, y, direction.0, direction.1, &word) {
                        all_match = false;
                        break;
                    }
                }

                if all_match {
                    count += 1;
                }
            }
        }
    }

    println!("Part 2: {count}");
}

pub fn run(input: &str) {
    part_1(input);
    part_2(input);
}
