

#[test]
pub fn test_jaccard_sim() {
    let graph = vec![
        vec![1, 2, 3],
        vec![2, 3, 4],
        vec![3, 4, 5],
        vec![4, 5],
    ];
    // Test jaccard_sim()
    let expected_output = vec![
        (0, 1, 0.5),
        (0, 2, 0.2),
        (0, 3, 0.0),
        (1, 0, 0.5),
        (1, 2, 0.5),
        (1, 3, 0.25),
        (2, 0, 0.2),
        (2, 1, 0.5),
        (2, 3, 0.6666667),
        (3, 0, 0.0),
        (3, 1, 0.25),
        (3, 2, 0.6666667),
    ];
    let actual_output = crate::jaccard_similarity::jaccard_sim(&graph);
    assert_eq!(actual_output, expected_output);

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