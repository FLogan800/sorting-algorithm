/// Sorts a data set using Gnome Sort
///
/// Average time complexity: O(n<sup>2</sup>)
///
/// # Examples
///
/// ```
/// use sorting_algorithm::gnome_sort;
///
/// fn main() {
///     let mut data = [3, 1, 2, 5, 4];
///     
///     gnome_sort::sort(&mut data);
///
///     assert_eq!(data, [1, 2, 3, 4, 5]);
/// }
/// ```
pub fn sort<T: Ord>(data: &mut [T]) {
    let len = data.len();

    let mut i = 0;

    while i < len {
        if i == 0 {
            i += 1;
        }

        if data[i] >= data[i - 1] {
            i += 1;
        } else {
            data.swap(i, i - 1);
            i -= 1;
        }
    }
}
