/// Sorts a data set using Exchange Sort
///
/// Average time complexity: O(n<sup>2</sup>)
///
/// # Example
///
/// ```
/// use sorting_algorithm::exchange_sort;
///
/// fn main() {
///     let mut data = [3, 1, 2, 5, 4];
///     
///     exchange_sort::sort(&mut data);
///
///     assert_eq!(data, [1, 2, 3, 4, 5]);
/// }
/// ```
pub fn sort<T: Ord>(data: &mut [T]) {
    if data.len() <= 1 {
        return;
    }

    let n = data.len();

    for i in 0..(n-1) {
        for j in (i+1)..n {
            if data[i] > data[j] {
                data.swap(i, j);
            }
        }
    }
}