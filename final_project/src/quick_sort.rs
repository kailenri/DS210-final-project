// The function takes a mutable slice of tuples as input
pub fn quicksort(arr: &mut [(i32, i32, f32)]) {
    let len = arr.len();
    // If the length of the slice is less than 2, it's already sorted
    if len < 2 {
        return;
    }

    // Choose the pivot value as the middle element of the slice
    let pivot = arr[len / 2].2;
    // Initialize indices for the left and right sub-arrays
    let mut i = 0;
    let mut j = len - 1;

    // Partition the array around the pivot value
    while i <= j {
        // Find the first element from the left that's >= pivot
        while arr[i].2 < pivot {
            i += 1;
        }
        // Find the first element from the right that's <= pivot
        while arr[j].2 > pivot {
            j -= 1;
        }
        // Swap the elements at indices i and j
        if i <= j {
            arr.swap(i, j);
            i += 1;
            j -= 1;
        }
    }

    // Recursively sort the left sub-array (if it's not empty)
    if j > 0 {
        quicksort(&mut arr[0..=j]);
    }

    // Recursively sort the right sub-array (if it's not empty)
    quicksort(&mut arr[i..]);
}