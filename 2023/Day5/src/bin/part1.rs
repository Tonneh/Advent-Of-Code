use std::fs;

fn lowest_location(contents: &str) -> u64 {
    let lines: Vec<&str> = contents.split("\r\n").collect();
    let mut seeds: Vec<u64>= lines[0].split_whitespace().filter_map(|num| num.parse().ok()).collect();
    let steps: Vec<&str> = contents.split("\r\n\r\n").skip(1).collect();
    for step in steps {
        let nums: Vec<&str> = step.split("\r\n").collect();
        let mut num_ranges: Vec<Vec<u64>> = Vec::new();
        for line in nums.iter().skip(1) {
            num_ranges.push(line.split(" ").filter_map(|num| num.parse().ok()).collect());
        }
        let mut new_seeds = Vec::new();
        for seed in seeds {
            let mut found = false;
            for range in num_ranges.iter() {
                let min = range[1];
                let max = range[1] + range[2];
                if min <= seed && seed <= max {
                    new_seeds.push(seed.abs_diff(range[1]) + range[0]);
                    found = true;
                    break;
                }
            }
            if !found {
                new_seeds.push(seed);
            }
        }
        seeds = new_seeds;
    }
    *seeds.iter().min().unwrap()
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    println!("{}", lowest_location(&contents));
}
