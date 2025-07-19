/// Sorts a data set using Selection Sort
///
/// Average time complexity: O(n<sup>2</sup>)
///
/// # Examples
///
/// ```
/// use sorting_algorithm::selection_sort;
///
/// fn main() {
///     let mut data = [3, 1, 2, 5, 4];
///     
///     selection_sort::sort(&mut data);
///
///     assert_eq!(data, [1, 2, 3, 4, 5]);
/// }
/// ```
pub fn sort<T: Ord>(data: &mut [T]) {
    let len = data.len();

    for i in 0..len {
        let mut min_index = i;

        for j in i + 1..len {
            if data[j] < data[min_index] {
                min_index = j;
            }
        }

        data.swap(i, min_index);
    }
}
