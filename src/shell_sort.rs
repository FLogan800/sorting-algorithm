/// Sorts a data set using Shell Sort
///
/// Average time complexity: O(n<sup>2</sup>)
///
/// # Examples
///
/// ```
/// use sorting_algorithm::shell_sort;
///
/// fn main() {
///     let mut data = [3, 1, 2, 5, 4];
///     
///     shell_sort::sort(&mut data);
///
///     assert_eq!(data, [1, 2, 3, 4, 5]);
/// }
/// ```
pub fn sort<T: Ord>(data: &mut [T]) {
    if data.len() <= 1 {
        return;
    }

    let n = data.len();

    let mut gap = n / 2;

    while gap > 0 {
        for i in gap..n {
            let mut j = i;

            while j >= gap && data[j] < data[j - gap] {
                data.swap(j, j - gap);
                j -= gap;
            }
        }

        gap /= 2;
    }
}
