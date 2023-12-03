use std::fs;

fn sum_of_part_nums(contents: &str) -> u32 {
    let mut matrix = Vec::new();
    for line in contents.lines() {
        let row: Vec<char> = line.chars().collect();
        matrix.push(row);
    }

    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut res = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] != '*' {
                continue;
            }
            let mut nums = Vec::new();
            for direction in &directions {
                let x = i as i32 + direction.0;
                let mut y = j as i32 + direction.1;
                if x < 0
                    || y < 0
                    || x >= matrix.len() as i32
                    || y >= matrix[x as usize].len() as i32
                    || !matrix[x as usize][y as usize].is_digit(10)
                {
                    continue;
                }

                while y >= 0 && matrix[x as usize][y as usize].is_digit(10) {
                    y -= 1;
                }
                y += 1;

                let mut num = String::new();
                while y < matrix[x as usize].len() as i32
                    && matrix[x as usize][y as usize].is_digit(10)
                {
                    num.push(matrix[x as usize][y as usize]);
                    matrix[x as usize][y as usize] = '.';
                    y += 1;
                }
                if let Ok(num) = num.parse::<u32>() {
                    nums.push(num);
                }
            }
            if nums.len() == 2 {
                res += nums[0] * nums[1];
            }
        }
    }
    res
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    println!("{}", sum_of_part_nums(&contents));
}
