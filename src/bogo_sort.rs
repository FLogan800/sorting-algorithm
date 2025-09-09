use fastrand;

/// Sorts a data set using Bogo Sort
///
/// Average time complexity: O(n*n!)
///
/// # Example
///
/// ```
/// use sorting_algorithm::bogo_sort;
///
/// fn main() {
///     let mut data = [3, 1, 2, 5, 4];
///     
///     bogo_sort::sort(&mut data);
///
///     assert_eq!(data, [1, 2, 3, 4, 5]);
/// }
/// ```
pub fn sort<T: Ord>(data: &mut [T]) {
    while !data.is_sorted() {
        fastrand::shuffle(data);
    }
}
