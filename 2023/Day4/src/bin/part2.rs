use std::collections::{HashMap, HashSet};
use std::fs;

fn total_points(contents: &str) -> u32 {
    let mut map = HashMap::new();
    for line in contents.lines() {
        let (card, nums) = line.split_once(':').unwrap();
        let (winning_nums, my_nums) = nums.split_once('|').unwrap();

        let winning_set: HashSet<u32> = winning_nums.split_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();

        let count = my_nums.split_whitespace()
           .filter_map(|num| num.parse::<u32>().ok())
           .filter(|num| winning_set.contains(num))
           .count() as u32;

        let card_num: String = card.chars().filter(|c| c.is_digit(10)).collect();
        let card_num: u32 = card_num.parse().unwrap();

        *map.entry(card_num).or_insert(0) += 1;
        for i in 0..count {
            *map.entry(card_num + 1 + i).or_insert(0) += map[&card_num];
        }
    }
    map.values().sum()
}


fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    println!("{}", total_points(&contents));
}
