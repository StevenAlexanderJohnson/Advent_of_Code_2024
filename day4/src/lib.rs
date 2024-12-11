use shared_types::AOCResult;

pub fn run(file_path: String, part_number: u32) -> AOCResult<u32> {
    match part_number {
        1 => part_1(file_path),
        2 => part_2(file_path),
        _ => Err("Invalid part number".into()),
    }
}

fn part_1(input_path: String) -> AOCResult<u32> {
    let input = std::fs::read_to_string(input_path)?
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let mut output = 0;
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if look_right(&input, x, y) {
                output += 1;
            }

            if look_down(&input, x, y) {
                output += 1;
            }

            if look_down_right(&input, x, y) {
                output += 1;
            }

            if look_down_left(&input, x, y) {
                output += 1;
            }
        }
    }

    Ok(output)
}


fn part_2(input_path: String) -> AOCResult<u32> {
    let input = std::fs::read_to_string(input_path)?
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let mut output = 0;

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if text_x_mas(&input, x, y) {
                output += 1;
            }
        }
    }

    Ok(output)
}

fn look_right(input: &[Vec<char>], x: usize, y: usize) -> bool {
    let substring = input[y].iter().skip(x).take(4).collect::<String>();

    if substring != "XMAS" && substring != "SAMX" {
        return false;
    }

    true
}

fn look_down(input: &[Vec<char>], x: usize, y: usize) -> bool {
    let substring = input
        .iter()
        .skip(y)
        .take(4)
        .map(|row| row[x])
        .collect::<String>();

    if substring != "XMAS" && substring != "SAMX" {
        return false;
    }

    true
}

fn look_down_right(input: &[Vec<char>], x: usize, y: usize) -> bool {
    if x + 3 >= input[y].len() || y + 3 >= input.len() {
        return false;
    }
    let substring = [
        input[y][x],
        input[y + 1][x + 1],
        input[y + 2][x + 2],
        input[y + 3][x + 3],
    ]
    .iter()
    .collect::<String>();

    if substring != "XMAS" && substring != "SAMX" {
        return false;
    }

    true
}

fn look_down_left(input: &[Vec<char>], x: usize, y: usize) -> bool {
    if x < 3 || y + 3 >= input.len() {
        return false;
    }

    let substring = [
        input[y][x],
        input[y + 1][x - 1],
        input[y + 2][x - 2],
        input[y + 3][x - 3],
    ]
    .iter()
    .collect::<String>();

    if substring != "XMAS" && substring != "SAMX" {
        return false;
    }

    true
}

struct XMasBlock {
    top_left: char,
    top_right: char,
    bottom_left: char,
    bottom_right: char,
}

impl XMasBlock {
    fn new(top_left: char, top_right: char, bottom_left: char, bottom_right: char) -> Self {
        Self {
            top_left,
            top_right,
            bottom_left,
            bottom_right,
        }
    }

    fn validate_x_mas(&self) -> bool {
        if ((self.top_left == 'M' && self.bottom_right == 'S')
            || (self.top_left == 'S' && self.bottom_right == 'M'))
            && ((self.top_right == 'M' && self.bottom_left == 'S')
                || (self.top_right == 'S' && self.bottom_left == 'M'))
        {
            true
        } else {
            false
        }
    }
}

fn text_x_mas(input: &[Vec<char>], x: usize, y: usize) -> bool {
    if x < 1 || y < 1 || x + 1 >= input[y].len() || y + 1 >= input.len() {
        return false;
    }
    if input[y][x] != 'A' {
        return false;
    }

    let x_mas_block = XMasBlock::new(
        input[y - 1][x - 1],
        input[y - 1][x + 1],
        input[y + 1][x - 1],
        input[y + 1][x + 1],
    );

    if !x_mas_block.validate_x_mas() {
        return false;
    }

    true
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_look_down() {
        let test_input: Vec<Vec<char>> = Vec::from([
            "....XXMAS.".chars().collect(),
            ".SAMXMS...".chars().collect(),
            "...S..A...".chars().collect(),
            "..A.A.MS.X".chars().collect(),
            "XMASAMX.MM".chars().collect(),
            "X.....XA.A".chars().collect(),
            "S.S.S.S.SS".chars().collect(),
            ".A.A.A.A.A".chars().collect(),
            "..M.M.M.MM".chars().collect(),
            ".X.X.XMASX".chars().collect(),
        ]);

        assert_eq!(look_down(&test_input, 0, 3), false);
        assert_eq!(look_down(&test_input, 0, 2), false);
        assert_eq!(look_down(&test_input, 0, 0), false);
        assert_eq!(look_down(&test_input, 9, 9), false);
        assert_eq!(look_down(&test_input, 9, 3), true);
    }

    #[test]
    fn test_look_right() {
        let test_input: Vec<Vec<char>> = Vec::from([
            "....XXMAS.".chars().collect(),
            ".SAMXMS...".chars().collect(),
            "...S..A...".chars().collect(),
            "..A.A.MS.X".chars().collect(),
            "XMASAMX.MM".chars().collect(),
            "X.....XA.A".chars().collect(),
            "S.S.S.S.SS".chars().collect(),
            ".A.A.A.A.A".chars().collect(),
            "..M.M.M.MM".chars().collect(),
            ".X.X.XMASX".chars().collect(),
        ]);

        assert_eq!(look_right(&test_input, 1, 0), false);
        assert_eq!(look_right(&test_input, 1, 1), true);
        assert_eq!(look_right(&test_input, 5, 0), true);
    }

    #[test]
    fn test_look_down_right() {
        let test_input: Vec<Vec<char>> = Vec::from([
            "....XXMAS.".chars().collect(),
            ".SAMXMS...".chars().collect(),
            "...S..A...".chars().collect(),
            "..A.A.MS.X".chars().collect(),
            "XMASAMX.MM".chars().collect(),
            "X.....XA.A".chars().collect(),
            "S.S.S.S.SS".chars().collect(),
            ".A.A.A.A.A".chars().collect(),
            "..M.M.M.MM".chars().collect(),
            ".X.X.XMASX".chars().collect(),
        ]);

        assert_eq!(look_down_right(&test_input, 4, 0), true);
        assert_eq!(look_down_right(&test_input, 5, 0), false);
        assert_eq!(look_down_right(&test_input, 0, 0), false);
        assert_eq!(look_down_right(&test_input, 0, 6), true);
    }

    #[test]
    fn test_look_down_left() {
        let test_input: Vec<Vec<char>> = Vec::from([
            "....XXMAS.".chars().collect(),
            ".SAMXMS...".chars().collect(),
            "...S..A...".chars().collect(),
            "..A.A.MS.X".chars().collect(),
            "XMASAMX.MM".chars().collect(),
            "X.....XA.A".chars().collect(),
            "S.S.S.S.SS".chars().collect(),
            ".A.A.A.A.A".chars().collect(),
            "..M.M.M.MM".chars().collect(),
            ".X.X.XMASX".chars().collect(),
        ]);

        assert_eq!(look_down_left(&test_input, 4, 0), false);
        assert_eq!(look_down_left(&test_input, 5, 0), false);
        assert_eq!(look_down_left(&test_input, 9, 0), false);
        assert_eq!(look_down_left(&test_input, 3, 2), true);
    }

    #[test]
    fn test_xmasblock_new() {
        let x_mas_block = XMasBlock::new('A', 'B', 'C', 'D');
        assert_eq!(x_mas_block.top_left, 'A');
        assert_eq!(x_mas_block.top_right, 'B');
        assert_eq!(x_mas_block.bottom_left, 'C');
        assert_eq!(x_mas_block.bottom_right, 'D');
    }

    #[test]
    fn test_xmasblock_validate() {
        let block = XMasBlock::new('M', 'M', 'S', 'S');
        assert_eq!(block.validate_x_mas(), true);

        let block = XMasBlock::new('M', 'S', 'S', 'M');
        assert_eq!(block.validate_x_mas(), false);

        let block = XMasBlock::new('S', 'M', 'M', 'S');
        assert_eq!(block.validate_x_mas(), false);

        let block = XMasBlock::new('S', 'S', 'M', 'M');

        assert_eq!(block.validate_x_mas(), true);

        let block = XMasBlock::new('A', 'B', 'C', 'D');
        assert_eq!(block.validate_x_mas(), false);

        let block = XMasBlock::new('M', 'S', 'M', 'S');
        assert_eq!(block.validate_x_mas(), true);
    }
}
