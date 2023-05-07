use std::collections::HashSet;

pub fn jacc(neighbors_i: &HashSet<i32>, neighbors_j: &HashSet<i32>) -> f32 {
    // Calculate the size of the intersection between the sets
    let intersection: f32 = neighbors_i.intersection(neighbors_j).count() as f32; 
    
    // Calculate the size of the union between the sets
    let union: f32 = neighbors_i.union(neighbors_j).count() as f32;
       
     // Calculate and return the Jaccard similarity coefficient
    intersection / union
}

pub fn jaccard_sim(graph: &Vec<Vec<i32>>) -> Vec<(i32, i32, f32)> {
     // Create a new vector to hold the similarity coefficients
    let mut sims: Vec<(i32, i32, f32)> = Vec::new();
    
    // Iterate over the vertices in the graph
    for (i_idx, i) in graph.iter().enumerate() {

        // Create a new `HashSet` containing the neighbors of the current vertex
        let set: HashSet<i32> = i.iter().copied().collect();

        // Iterate over the remaining vertices in the graph
        for (j_idx, j) in graph.iter().enumerate() {
            //Skips comparisons to self
            if i == j {
                continue;
            }
            // Create a new `HashSet` containing the other vertex
            let vert_sets: HashSet<i32> = j.iter().copied().collect();
            let simnumber: f32 = jacc(&set, &vert_sets);
            sims.push((i_idx as i32, j_idx as i32, simnumber));
        }
    }
    sims
}