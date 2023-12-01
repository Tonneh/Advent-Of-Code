use std::collections::HashMap;
use std::fs;

fn get_calibration_values(content: &str) -> u32 {
    let nums_map = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let lines = content.lines();
    let mut res = 0;
    for line in lines {
        let mut nums = Vec::new();
        for (i, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                nums.push(char.to_digit(10).unwrap());
            }
            for num in nums_map.iter() {
                if line[i..].starts_with(num.0) {
                    nums.push(*num.1);
                }
            }
        }
        res += format!("{}{}", nums.first().unwrap(), nums.last().unwrap())
            .parse::<u32>()
            .unwrap();
    }
    res
}

fn main() {
    let contents = fs::read_to_string("input1.txt").unwrap();
    println!("{}", get_calibration_values(&contents));
}
