// Import necessary modules
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read(file_path: &str) -> Vec<Vec<i32>> {
    let file: File = File::open(file_path).unwrap();
    let reader: BufReader<File> = BufReader::new(file);

    let mut data: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let nums: Vec<i32> = line.unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let (idx, val) = (nums[0] as usize, nums[1]);

        while data.len() <= idx {
            data.push(Vec::new());
        }

        data[idx].push(val);
    }

    data
}