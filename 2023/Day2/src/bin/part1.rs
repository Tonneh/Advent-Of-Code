use std::collections::HashMap;
use std::fs;

fn get_sum_of_ids(contents: &str) -> u32 {
    let map = HashMap::from([
        ("red", 12u32),
        ("green", 13u32),
        ("blue", 14u32),
    ]);
    let mut sum = 0;
    for line in contents.lines() {
        let mut r#break = false;
        let (game_id_str, colors_str) = line.split_once(':').unwrap();

        let sets: Vec<&str> = colors_str.trim().split(';').collect();
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
                if map.get(&*color).unwrap() < &count.parse::<u32>().unwrap() {
                    r#break = true;
                    break;
                }
            }
            if r#break {
                break;
            }
        }
        if !r#break {
            let mut game_id = String::new();
            for char in game_id_str.chars() {
                if char.eq(&':') {
                    break;
                }
                if char.is_digit(10) {
                    game_id.push(char);
                }
            }
            sum += game_id.parse::<u32>().unwrap();
        }
    }
    sum
}

fn main() {
    let contents = fs::read_to_string("input1.txt").unwrap();

    println!("{}", get_sum_of_ids(&contents));
}
