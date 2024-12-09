#[derive(Debug, Clone)]
struct FileIndicesRange {
    start: i64,
    length: i64,
}

type FileIndices = Vec<FileIndicesRange>;

#[derive(Debug, Clone)]
struct File {
    idx: i64,
    indices: FileIndices,
}

type Disk = Vec<File>;

fn parse_input(input: &str) -> Disk {
    let mut cur_file_idx = 0;
    let mut cur_disk_idx = 0;
    let mut disk: Disk = Vec::new();

    let numbers = input
        .trim()
        .chars()
        .map(|x| x.to_string().parse::<i64>().expect("Has to be number"));

    let mut is_free = false;

    for num in numbers {
        if !is_free {
            let file = File {
                idx: cur_file_idx,
                indices: vec![FileIndicesRange {
                    start: cur_disk_idx,
                    length: num,
                }],
            };

            cur_file_idx += 1;

            disk.push(file);
        }

        is_free = !is_free;
        cur_disk_idx += num;
    }

    disk
}

fn calculate_checksum_range(file_idx: i64, range: &FileIndicesRange) -> i64 {
    let mut checksum = 0;
    let mut length = range.length;
    let start = range.start;

    if length % 2 == 0 {
        checksum += start + length - 1;
        length -= 1;
    }

    checksum += (start + (length - 1) / 2) * length;

    checksum *= file_idx;

    checksum
}

fn calculate_checksum_file(file: &File) -> i64 {
    file.indices
        .iter()
        .map(|indice| calculate_checksum_range(file.idx, indice))
        .sum()
}

fn calculate_checksum_disk(disk: &Disk) -> i64 {
    disk.iter().map(calculate_checksum_file).sum()
}

fn part_1(input: &str) {
    let mut disk = parse_input(input);

    let mut cur_file_idx = 0;
    let mut last_file_idx = disk.len() - 1;
    let mut free_space: i64 = 0;
    let mut cur_end_disk_idx: i64 = 0;

    while cur_file_idx <= last_file_idx {
        if free_space == 0 {
            let file_range = &disk[cur_file_idx].indices[0];

            if file_range.start > cur_end_disk_idx {
                free_space = file_range.start - cur_end_disk_idx;
            } else {
                cur_end_disk_idx = file_range.start + file_range.length;
                cur_file_idx += 1;
            }
        } else {
            let original_indices_start = disk[last_file_idx].indices[0].start;
            let original_indices_length = disk[last_file_idx].indices[0].length;

            if original_indices_length > free_space {
                disk[last_file_idx].indices.push(FileIndicesRange {
                    start: cur_end_disk_idx,
                    length: free_space,
                });
                disk[last_file_idx].indices[0] = FileIndicesRange {
                    start: original_indices_start,
                    length: original_indices_length - free_space,
                };
                cur_end_disk_idx += free_space;
                free_space = 0;
            } else {
                disk[last_file_idx].indices.push(FileIndicesRange {
                    start: cur_end_disk_idx,
                    length: original_indices_length,
                });
                disk[last_file_idx].indices.remove(0);
                cur_end_disk_idx += original_indices_length;
                free_space -= original_indices_length;
                last_file_idx -= 1;
            }
        }
    }

    let checksum = calculate_checksum_disk(&disk);

    println!("Part1: {checksum}");
}

fn part_2(input: &str) {
    let disk = parse_input(input);
    let mut final_disk = disk.clone();

    for file_idx in (1..disk.len()).rev() {
        let file = &disk[file_idx];
        let start = file.indices[0].start;
        let length = file.indices[0].length;

        let mut moved = 0;

        for place_idx in 1..final_disk.len() {
            let prev_item = &final_disk[place_idx - 1].indices[0];
            let next_item = &final_disk[place_idx].indices[0];
            let space = next_item.start - (prev_item.start + prev_item.length);
            let space_start = prev_item.start + prev_item.length;

            if space_start >= start {
                break;
            }

            if space >= length {
                final_disk.insert(
                    place_idx,
                    File {
                        idx: file.idx,
                        indices: vec![FileIndicesRange {
                            start: prev_item.start + prev_item.length,
                            length,
                        }],
                    },
                );

                moved = place_idx + 1;
                break;
            }
        }

        if moved > 0 {
            for cur_idx in moved..final_disk.len() {
                if final_disk[cur_idx].idx == file.idx {
                    final_disk.remove(cur_idx);
                    break;
                }
            }
        }
    }

    let checksum = calculate_checksum_disk(&final_disk);

    println!("Part 2: {checksum}");
}

pub fn run(input: &str) {
    part_1(input);
    part_2(input);
}
