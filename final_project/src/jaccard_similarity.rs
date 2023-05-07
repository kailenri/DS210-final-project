use std::collections::HashSet;

pub fn jacc(neighbors_i: &HashSet<i32>, neighbors_j: &HashSet<i32>) -> f32 {
    let intersection: f32 = neighbors_i.intersection(neighbors_j).count() as f32;
    let union: f32 = neighbors_i.union(neighbors_j).count() as f32;
    intersection / union
}

pub fn jaccard_sim(graph: &Vec<Vec<i32>>) -> Vec<(i32, i32, f32)> {
    let mut sims: Vec<(i32, i32, f32)> = Vec::new();
    for (i_idx, i) in graph.iter().enumerate() {
        let set: HashSet<i32> = i.iter().copied().collect();
        for (j_idx, j) in graph.iter().enumerate() {
            if i == j {
                continue;
            }
            let neighbor_sets: HashSet<i32> = j.iter().copied().collect();
            let simnumber: f32 = jacc(&set, &neighbor_sets);
            sims.push((i_idx as i32, j_idx as i32, simnumber));
        }
    }
    sims
}