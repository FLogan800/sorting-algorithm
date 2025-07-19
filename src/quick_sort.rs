use fastrand;

/// Sorts a data set using Quick Sort
///
/// Average time complexity: O(n logn)
///
/// # Examples
///
/// ```
/// use sorting_algorithm::quick_sort;
///
/// fn main() {
///     let mut data = [3, 1, 2, 5, 4];
///     
///     quick_sort::sort(&mut data);
///
///     assert_eq!(data, [1, 2, 3, 4, 5]);
/// }
/// ```
pub fn sort<T: Ord>(data: &mut [T]) {
    if data.len() > 1 {
        let pivot = partition(data);

        sort(&mut data[..pivot]);
        sort(&mut data[pivot + 1..]);
    }
}

fn partition<T: Ord>(data: &mut [T]) -> usize {
    let n = data.len();
    let pivot = fastrand::usize(0..data.len());
    data.swap(pivot, n - 1);

    let mut i = 0;

    for j in 0..n - 1 {
        if data[j] <= data[n - 1] {
            data.swap(i, j);
            i += 1;
        }
    }

    data.swap(i, n - 1);
    i
}
