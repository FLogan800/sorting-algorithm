/// Sorts a data set using Cocktail Shaker Sort
///
/// Average time complexity: O(n<sup>2</sup>)
///
/// # Examples
///
/// ```
/// use sorting_algorithm::cocktail_shaker_sort;
///
/// fn main() {
///     let mut data = [3, 1, 2, 5, 4];
///     
///     cocktail_shaker_sort::sort(&mut data);
///
///     assert_eq!(data, [1, 2, 3, 4, 5]);
/// }
/// ```
pub fn sort<T: Ord>(data: &mut [T]) {
    if data.len() <= 1 {
        return;
    }

    let mut low = 0;
    let mut high = data.len() - 1;

    while low < high {
        let mut swapped = false;

        for i in low..high {
            if data[i] > data[i + 1] {
                data.swap(i, i + 1);
                swapped = true;
            }
        }
        high -= 1;

        if !swapped {
            return;
        }

        for i in (low..high).rev() {
            if data[i] > data[i + 1] {
                data.swap(i, i + 1);
                swapped = true;
            }
        }
        low += 1;

        if !swapped {
            return;
        }
    }
}
