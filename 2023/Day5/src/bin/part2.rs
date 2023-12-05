use std::cmp::{max, min};
use std::fs;

fn lowest_location(contents: &str) -> u64 {
    let lines: Vec<&str> = contents.split("\r\n").collect();
    let seeds: Vec<u64>= lines[0].split_whitespace().filter_map(|num| num.parse().ok()).collect();
    let mut seed_pairs = Vec::new();
    for seed in seeds.chunks(2) {
        seed_pairs.push((seed[0], seed[0] + seed[1]));
    }
    let steps: Vec<&str> = contents.split("\r\n\r\n").skip(1).collect();
    for step in steps {
        let nums: Vec<&str> = step.split("\r\n").collect();
        let mut num_ranges: Vec<Vec<u64>> = Vec::new();
        for line in nums.iter().skip(1) {
            num_ranges.push(line.split(" ").filter_map(|num| num.parse().ok()).collect());
        }

        let mut new_seeds = Vec::new();
        while !seed_pairs.is_empty() {
            let (low, high) = seed_pairs.pop().unwrap();
            let mut found = false;
            for num_range in num_ranges.iter() {
                let src = num_range[1];
                let dst = num_range[0];
                let increment = num_range[2];

                let mapped_start = max(low, src);
                let mapped_end = min(src + increment, high);
                if mapped_start < mapped_end {
                    new_seeds.push(((mapped_start - src + dst), (mapped_end - src + dst)));
                    if mapped_start > low {
                        seed_pairs.push((low, mapped_start));
                    }
                    if mapped_end < high {
                        seed_pairs.push((mapped_end, high));
                    }
                    found = true;
                    break;
                }
            }
            if !found {
                new_seeds.push((low, high));
            }
        }
        seed_pairs = new_seeds;
    }
    let mut min_val = u64::MAX;
    for seed_pair in seed_pairs {
        min_val = min(min_val, seed_pair.0);
    }
    min_val
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    println!("{}", lowest_location(&contents));
}
