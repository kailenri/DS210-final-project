#[test]
pub fn test_jaccard_sim() {
    let graph: Vec<Vec<i32>> = vec![
        vec![],
        vec![2, 3],
        vec![1, 3, 4],
        vec![2, 4],
        vec![3],
    ];
    let sims = crate::jaccard_similarity::jaccard_sim(&graph);
    assert_eq!(sims.len(), 6);

    assert_eq!(sims[0], (1, 2, 0.25));
    assert_eq!(sims[1], (1, 3, 0.33333334));
    assert_eq!(sims[2], (1, 4, 0.5));
}

#[test]
pub fn test_quicksort() {
    let mut arr = [(3, 2, 0.5), (1, 4, 0.2), (2, 1, 0.8), (4, 3, 0.1)];
    crate::quick_sort::quicksort(&mut arr);
    assert_eq!(arr, [(4, 3, 0.1), (1, 4, 0.2), (3, 2, 0.5), (2, 1, 0.8)]);

    let mut arr = [(3, 2, 0.5), (1, 4, 0.2)];
    crate::quick_sort::quicksort(&mut arr);
    assert_eq!(arr, [(1, 4, 0.2), (3, 2, 0.5)]);

    let mut arr: [(i32, i32, f32); 0] = [];
    crate::quick_sort::quicksort(&mut arr);
    assert_eq!(arr, []);
}