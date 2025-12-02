pub fn run(input: &str) {
    let numbers = input.lines()
        .map(|line| {
            let direction = line.chars().next().unwrap();
            let number_str = &line[1..];
            let num: i32 = number_str.parse().unwrap();
            
            match direction {
                'L' => -num,
                'R' => num,
                _ => panic!("Invalid direction: {}", direction),
            }
        });

    let mut dial = 50;
    let mut part_one = 0;
    let mut part_two = 0;

    for number in numbers {
        if number > 0 {
            part_two += (dial + number) / 100;
        } else if number < 0 {
            let negated = (100 - dial) % 100;
            part_two += (negated - number) / 100;
        }

        dial = (dial + number).rem_euclid(100);

        if dial == 0 {
            part_one += 1;
        }
    }

    println!("Output Part 1: {part_one}");
    println!("Output Part 2: {part_two}");
}
