use std::collections::HashMap;

use shared_types::AOCResult;
pub fn run(file_path: String, part_number: u32) -> AOCResult<u32> {
    match part_number {
        1 => part_1(file_path),
        2 => part_2(file_path),
        _ => Err("Invalid part number".into()),
    }
}

fn part_1(file_path: String) -> AOCResult<u32> {
    let input = std::fs::read_to_string(file_path)?;

    let (mut left, mut right): (Vec<u32>, Vec<u32>) =
        input
            .lines()
            .fold((Vec::new(), Vec::new()), |(mut list1, mut list2), line| {
                let mut split_line = line
                    .split_whitespace()
                    .into_iter()
                    .map(|x| x.parse::<u32>().unwrap());

                list1.push(split_line.next().unwrap());
                list2.push(split_line.next().unwrap());
                return (list1, list2);
            });
    left.sort();
    right.sort();

    Ok(left
        .iter()
        .zip(right)
        .map(|(x, y)| x.abs_diff(y))
        .sum::<u32>())
}

fn part_2(file_path: String) -> AOCResult<u32> {
    let input = std::fs::read_to_string(file_path)?;

    let (left, right): (Vec<u32>, HashMap<u32, u32>) = input.lines().fold(
        (Vec::new(), HashMap::new()),
        |(mut list1, mut map), line| {
            let mut split_line = line
                .split_whitespace()
                .into_iter()
                .map(|x| x.parse::<u32>().unwrap());

            list1.push(split_line.next().unwrap());
            *map.entry(split_line.next().unwrap()).or_insert(0) += 1;
            return (list1, map);
        },
    );

    Ok(left.iter().map(|x| {
        x * right.get(x).unwrap_or(&0)
    }).sum())
}
