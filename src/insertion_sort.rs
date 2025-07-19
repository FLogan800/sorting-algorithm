/// Sorts a data set using Insertion Sort
///
/// Average time complexity: O(n<sup>2<sup>)
///
/// # Examples
///
/// ```
/// use sorting_algorithm::insertion_sort;
///
/// fn main() {
///     let mut data = [3, 1, 2, 5, 4];
///     
///     insertion_sort::sort(&mut data);
///
///     assert_eq!(data, [1, 2, 3, 4, 5]);
/// }
/// ```
pub fn sort<T: Ord>(data: &mut [T]) {
    for i in 1..data.len() {
        let mut j = i;

        while j > 0 && data[j] < data[j - 1] {
            data.swap(j, j - 1);
            j -= 1;
        }
    }
}
