/// Sorts a data set using Comb Sort
///
/// Average time complexity: O(n logn)
///
/// # Examples
///
/// ```
/// use sorting_algorithm::comb_sort;
///
/// fn main() {
///     let mut data = [3, 1, 2, 5, 4];
///     
///     comb_sort::sort(&mut data);
///
///     assert_eq!(data, [1, 2, 3, 4, 5]);
/// }
/// ```
pub fn sort<T: Ord>(data: &mut [T]) {
    if data.is_empty() {
        return;
    }

    let mut gap = data.len();
    let mut swapped = true;

    while gap != 1 || swapped {
        gap = get_next_gap(gap);

        swapped = false;

        for i in 0..data.len() - gap {
            if data[i] > data[i + gap] {
                data.swap(i, i + gap);
                swapped = true;
            }
        }
    }
}

fn get_next_gap(gap: usize) -> usize {
    let gap = (gap * 10) / 13;

    if gap < 1 {
        return 1;
    }

    gap
}
