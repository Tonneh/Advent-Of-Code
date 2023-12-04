use std::collections::HashSet;
use std::fs;

fn total_points(contents: &str) -> i32 {
    let mut res = 0;
    for line in contents.lines() {
        let (_card, nums) = line.split_once(':').unwrap();
        let (winning_nums, my_nums) = nums.split_once('|').unwrap();
        let mut my_nums = my_nums.to_string();
        my_nums.push(' ');

        let winning_set: HashSet<u32> = winning_nums.split_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();

        let count = my_nums.split_whitespace()
           .filter_map(|num| num.parse::<u32>().ok())
           .filter(|num| winning_set.contains(num))
           .count() as i32;

        if count - 1 < 0 {
            continue;
        }
        res += 2_i32.pow((count - 1) as u32);
    }
    res
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    println!("{}", total_points(&contents));
}
