use std::cmp::max;
use std::collections::HashMap;
use std::fs;

fn get_powers(contents: &str) -> u32 {
    let mut sum = 0;
    for line in contents.lines() {
        let mut current_max = HashMap::from([
            ("red".to_string(), 1u32),
            ("green".to_string(), 1u32),
            ("blue".to_string(), 1u32),
        ]);

        let (_, colors) = line.split_at(line.find(':').unwrap());
        let sets: Vec<&str> = colors.trim().split(';').collect();
        for set in sets {
            let count_and_colors: Vec<&str> = set.split(',').collect();
            for count_and_color in count_and_colors {
                let mut color = String::new();
                let mut count = String::new();
                for char in count_and_color.chars() {
                    if char.is_digit(10) {
                        count.push(char);
                    }
                    if char.is_alphabetic() {
                        color.push(char);
                    }
                }
                let count: u32 = count.parse().unwrap();
                current_max.insert(color.clone(), max(*current_max.get(&*color).unwrap(), count));
            }
        }
        let mut power = 1;
        for (_, value) in current_max {
            power *= value;
        }
        sum += power;
    }
    sum
}

fn main() {
    let contents = fs::read_to_string("input1.txt").unwrap();

    println!("{}", get_powers(&contents));
}
