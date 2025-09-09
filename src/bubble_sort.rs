/// Sorts a data set using Bubble Sort
///
/// Average time complexity: O(n<sup>2</sup>)
///
/// # Example
///
/// ```
/// use sorting_algorithm::bubble_sort;
///
/// fn main() {
///     let mut data = [3, 1, 2, 5, 4];
///     
///     bubble_sort::sort(&mut data);
///
///     assert_eq!(data, [1, 2, 3, 4, 5]);
/// }
/// ```
pub fn sort<T: Ord>(data: &mut [T]) {
    if data.len() <= 1 {
        return;
    }

    let n = data.len();

    for i in 0..n {
        let mut swapped = false;

        for j in 0..(n - i - 1) {
            if data[j] > data[j + 1] {
                data.swap(j, j + 1);
                swapped = true;
            }
        }

        if !swapped {
            return;
        }
    }
}
