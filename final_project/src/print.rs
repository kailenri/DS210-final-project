pub fn best_and_worst (sorted: &Vec<(i32, i32, f32)>) {
    let (x, y, z) = sorted[0];
    println!("the two LEAST similar points are {} and {} with a similarity of {}", x, y, z);
    
    let (x, y, z) = sorted[sorted.len() - 1];
    println!("the two MOST similar points are {} and {} with a similarity of {}", x, y, z);

}