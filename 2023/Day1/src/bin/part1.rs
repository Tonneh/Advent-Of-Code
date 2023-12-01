use std::fs;

fn get_calibration_values(content: &str) -> u32 {
    let lines = content.lines();
    let mut res = 0;
    for line in lines {
        let mut nums = Vec::new();
        for char in line.chars() {
            if char.is_digit(10) {
                nums.push(char.to_digit(10).unwrap());
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
