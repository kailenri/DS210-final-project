// Import necessary modules
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read(file_path: &str) -> Vec<Vec<i32>> {
    // Open the file at the specified path and create a buffered reader
    let file: File = File::open(file_path).unwrap();
    let reader: BufReader<File> = BufReader::new(file);

    // Create a new vector to hold the adjacency lists
    let mut data: Vec<Vec<i32>> = Vec::new();
    
    // Iterate over each line in the file
    for line in reader.lines() {

        // Split the line into a vector of integers
        let nums: Vec<i32> = line.unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        // Extract the vertex index and its neighbors from the vector of integers
        let (idx, val) = (nums[0] as usize, nums[1]);

        // Ensure that the adjacency list vector for the current vertex exists
        while data.len() <= idx {
            data.push(Vec::new());
        }

        // Add the neighbor to the adjacency list for the current vertex
        data[idx].push(val);
    }

    data
}