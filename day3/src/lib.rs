use regex::Regex;
use shared_types::AOCResult;

pub fn run(file_path: String, part_number: u32) -> AOCResult<u32> {
    match part_number {
        1 => part_1(&file_path),
        2 => part_2(&file_path),
        _ => Err("Invalid part number".into()),
    }
}

pub fn part_1(file_path: &str) -> AOCResult<u32> {
    let input = std::fs::read_to_string(file_path).unwrap();
    Ok(find_multiplication_numbers(input.as_str())?
        .iter()
        .map(|(x, y)| x * y)
        .sum())
}

pub fn part_2(file_path: &str) -> AOCResult<u32> {
    let input = std::fs::read_to_string(file_path).unwrap();
    let mut mul_enabled = true;
    let mut output = 0;
    let valid_commands = find_valid_commands(input.as_str());
    for command in valid_commands {
        match command {
            "do()" => {
                mul_enabled = true;
            }
            "don't()" => {
                mul_enabled = false;
            }
            x => {
                if mul_enabled {
                    let (x, y) = find_multiplication_numbers(x)?[0];
                    output += x * y;
                }
            }
        }
    }
    Ok(output)
}

fn find_valid_commands(input: &str) -> Vec<&str> {
    let re = Regex::new(r"do\(\)|mul\((\d+),(\d+)\)|don't\(\)").unwrap();
    re.find_iter(&input).map(|mat| mat.as_str()).collect()
}

fn find_multiplication_numbers(input: &str) -> AOCResult<Vec<(u32, u32)>> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)")?;
    let captures = re
        .captures_iter(&input)
        .map(|cap| {
            let x = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let y = cap.get(2).unwrap().as_str().parse::<u32>().unwrap();

            (x, y)
        })
        .collect::<Vec<(u32, u32)>>();
    Ok(captures)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_valid_commands() {
        let input = "mul(1,2) mul(3,4) mul(5,6)";
        let expected = vec!["mul(1,2)", "mul(3,4)", "mul(5,6)"];
        assert_eq!(find_valid_commands(input), expected);
        let input = "xmul(1,2!),mul(3,4)&mul(5,6))";
        let expected = vec!["mul(3,4)", "mul(5,6)"];
        assert_eq!(find_valid_commands(input), expected);
    }

    #[test]
    fn test_find_multiplication_numbers() {
        let input = "mul(1,2) mul(3,4) mul(5,6)";
        let expected = vec![(1, 2), (3, 4), (5, 6)];
        assert_eq!(find_multiplication_numbers(input), expected);
        let input = "xmul(1,2!),mul(3,4)&mul(5,6))";
        let expected = vec![(3, 4), (5, 6)];
        assert_eq!(find_multiplication_numbers(input), expected);
    }
}
