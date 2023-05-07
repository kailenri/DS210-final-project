// Import necessary modules
use std::fs::File;
use std::io::{BufRead, BufReader};

// Define the function that takes a path as a parameter
pub fn read(path: &str) -> Vec<Vec<i32>> {
    // Initialize an empty vector to store the result
    let mut result: Vec<Vec<i32>> = Vec::new();

    // Open the file at the specified path
    let file: File = File::open(path).expect("Could not open file");

    // Initialize a buffer to read lines from the file
    let buffer: std::io::Lines<BufReader<File>> = BufReader::new(file).lines();

    // Loop through each line in the buffer
    for line in buffer {
        // Extract the line as a string
        let line_str = line.expect("Error reading");

        // Parse the line as a vector of integers
        let vector_from_txt: Vec<i32> = line_str
            .trim()
            .split(" ")
            .map(|s: &str| s.parse().expect("Error parsing"))
            .collect();

        // Add the parsed vector to the result
        result.push(vector_from_txt);
    }

    // Return the parsed vectors as a Vec<Vec<i32>>
    result
}