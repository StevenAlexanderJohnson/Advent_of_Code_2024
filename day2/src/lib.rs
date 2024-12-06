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
    let mut output = 0;

    input.lines().for_each(|line| {
        let parts = line
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        if parts
            .windows(2)
            .all(|w| w[0] <= w[1] && w[0].abs_diff(w[1]) <= 3 && w[0].abs_diff(w[1]) > 0)
            || parts
                .windows(2)
                .all(|w| w[0] >= w[1] && w[0].abs_diff(w[1]) <= 3 && w[0].abs_diff(w[1]) > 0)
        {
            output += 1;
        }
    });

    Ok(output)
}

fn part_2(file_path: String) -> AOCResult<u32> {
    let input = std::fs::read_to_string(file_path)?;

    let lines = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .filter(|line| verify_line(line))
        .collect::<Vec<Vec<u32>>>();

    Ok(lines.len() as u32)
}

fn remove_anomaly(line: &Vec<u32>) -> (Vec<u32>, Option<Vec<u32>>) {
    let ascending = line[0] < line[1];
    if let Some(position) = line.windows(2).position(|w| {
        let (x, y) = (w[0], w[1]);
        if ascending {
            x >= y || x.abs_diff(y) > 3
        } else {
            x <= y || x.abs_diff(y) > 3
        }
    }) {
        (
            line.iter()
                .enumerate()
                .filter(|&(i, _)| i != position + 1)
                .map(|(_, &x)| x)
                .collect(),
            Some(
                line.iter()
                    .enumerate()
                    .filter(|&(i, _)| i != position)
                    .map(|(_, &x)| x)
                    .collect(),
            ),
        )
    } else {
        (line.clone(), None)
    }
}

fn verify_line(line: &Vec<u32>) -> bool {
    let (line1, line2) = remove_anomaly(&line);
    let ascending = line1.first().unwrap() < line1.last().unwrap();
    if (ascending
        && line1
            .windows(2)
            .all(|w| w[0] < w[1] && w[0].abs_diff(w[1]) <= 3 && w[0].abs_diff(w[1]) > 0))
        || (!ascending
            && line1
                .windows(2)
                .all(|w| w[0] > w[1] && w[0].abs_diff(w[1]) <= 3 && w[0].abs_diff(w[1]) > 0))
    {
        true
    } else if let Some(line2) = line2 {
        if (ascending
            && line2
                .windows(2)
                .all(|w| w[0] < w[1] && w[0].abs_diff(w[1]) <= 3 && w[0].abs_diff(w[1]) > 0))
            || (!ascending
                && line2
                    .windows(2)
                    .all(|w| w[0] > w[1] && w[0].abs_diff(w[1]) <= 3 && w[0].abs_diff(w[1]) > 0))
        {
            true
        } else {
            false
        }
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_line_ascending() {
        let line = [1, 2, 3, 4, 5].to_vec();
        assert_eq!(verify_line(&line), true);

        let line = [1, 7, 2, 3, 4, 5].to_vec();
        assert_eq!(verify_line(&line), true);

        let line = [1, 2, 5, 6, 7].to_vec();
        assert_eq!(verify_line(&line), true);

        let line = [1, 2, 6, 7, 8].to_vec();
        assert_eq!(verify_line(&line), false);
    }

    #[test]
    fn test_verify_line_descending() {
        let line = [8, 7, 6, 5, 4].to_vec();
        assert_eq!(verify_line(&line), true);

        let line = [8, 7, 6, 5, 1].to_vec();
        assert_eq!(verify_line(&line), true);

        let line = [8, 7, 6, 5, 1, 2].to_vec();
        assert_eq!(verify_line(&line), true);

        let line = [8, 7, 2, 1, 0].to_vec();
        assert_eq!(verify_line(&line), false);

        let line = [8, 2, 1, 0].to_vec();
        assert_eq!(verify_line(&line), true);

        let line = [5, 4, 3, 2, 1, 9].to_vec();
        assert_eq!(verify_line(&line), true);
    }

    #[test]
    fn test_remove_anomaly_ascending() {
        let line = [1, 2, 7, 3, 4, 5].to_vec();
        assert_eq!(
            remove_anomaly(&line),
            ([1, 2, 3, 4, 5].to_vec(), Some([1, 7, 3, 4, 5].to_vec()))
        );

        let line = [1, 2, 7, 3, 4, 5].to_vec();
        assert_eq!(
            remove_anomaly(&line),
            ([1, 2, 3, 4, 5].to_vec(), Some([1, 7, 3, 4, 5].to_vec()))
        );

        let line = [1, 2, 7, 8, 3, 4, 5].to_vec();
        assert_eq!(
            remove_anomaly(&line),
            (
                [1, 2, 8, 3, 4, 5].to_vec(),
                Some([1, 7, 8, 3, 4, 5].to_vec())
            )
        );

        let line = [1, 2, 3, 4, 5, 6].to_vec();
        assert_eq!(remove_anomaly(&line), ([1, 2, 3, 4, 5, 6].to_vec(), None));

        let line = [9, 1, 2, 3, 4, 5].to_vec();
        assert_eq!(
            remove_anomaly(&line),
            ([9, 2, 3, 4, 5].to_vec(), Some([1, 2, 3, 4, 5].to_vec()))
        );
    }

    #[test]
    fn test_remove_anomaly_descending() {
        let line = [8, 7, 2, 1, 0].to_vec();
        assert_eq!(
            remove_anomaly(&line),
            ([8, 7, 1, 0].to_vec(), Some([8, 2, 1, 0].to_vec()))
        );

        let line = [8, 6, 4, 4, 1].to_vec();
        assert_eq!(
            remove_anomaly(&line),
            ([8, 6, 4, 1].to_vec(), Some([8, 6, 4, 1].to_vec()))
        );

        let line = [9, 1, 2, 3, 4, 5].to_vec();
        assert_eq!(
            remove_anomaly(&line),
            ([9, 2, 3, 4, 5].to_vec(), Some([1, 2, 3, 4, 5].to_vec()))
        );

        let line = [6, 5, 4, 3, 2, 1].to_vec();
        assert_eq!(remove_anomaly(&line), ([6, 5, 4, 3, 2, 1].to_vec(), None));

        let line = [5, 4, 3, 2, 1, 9].to_vec();
        assert_eq!(
            remove_anomaly(&line),
            ([5, 4, 3, 2, 1].to_vec(), Some([5, 4, 3, 2, 9].to_vec()))
        );

        let line = [1, 2, 5, 4, 3, 2, 1].to_vec();
        assert_eq!(
            remove_anomaly(&line),
            ([1, 2, 5, 3, 2, 1].to_vec(), Some([1, 2, 4, 3, 2, 1].to_vec()))
        );
    }
}
