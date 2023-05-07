use std::collections::HashSet;

pub fn jacc(neighbors_i: &HashSet<i32>, neighbors_j: &HashSet<i32>) -> f32 {
    let intersection: f32 = neighbors_i.intersection(neighbors_j).count() as f32; //calculates the intersection between the two Vertecies and their connections
    let union: f32 = neighbors_i.union(neighbors_j).count() as f32; //calculates the union between the two Vertecies and their connections
    intersection/union //returns the Jaccard similarity 
}

pub fn jaccard_sim(graph: &Vec<Vec<i32>>) -> Vec<(i32, i32, f32)>{
    let mut sims: Vec<(i32, i32, f32)> = Vec::new(); //creates a new vector that will contain the two points, and their similarity
    let neighbor_sets: Vec<HashSet<i32>> = graph.iter().map(|neighbors: &Vec<i32>| neighbors.iter().copied().collect()).collect(); //turns the graph into a hashset 
    
    //for each point the similarity is calculated for every point in the graph. 
    for i in 1..graph.len(){
        for j in i+1..graph.len() {
            let simnumber: f32 = jacc( &neighbor_sets[i], &neighbor_sets[j]); //passes in the two points into the Jacc function so thier similarity can be calculated 
            sims.push((i as i32, j as i32, simnumber)); //adds the similarity to the vector
        }
    }
    sims 
}