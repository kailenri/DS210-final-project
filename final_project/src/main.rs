mod jaccard_similarity;
mod print;
mod quick_sort;
mod read;
mod tests;

fn main() {
    // Read the graph from the file
    let graph: Vec<Vec<i32>> = crate::read::read("facebook.txt");
    println!("Graph has been read");

    // Calculate the Jaccard similarity between every pair of nodes
    let mut calc: Vec<(i32, i32, f32)> = crate::jaccard_similarity::jaccard_sim(&graph);
    println!("Jaccard similarity has been calculated");
    // Sort the similarities from lowest to highest
    crate::quick_sort::quicksort(&mut calc);
    println!("Scores have been sorted");

    // Print out the least and most similar pairs of nodes
    crate::print::best_and_worst(&calc);
}