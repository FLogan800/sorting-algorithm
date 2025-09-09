/// Sorts a data set using Merge Sort
///
/// Average time complexity: O(n logn)
///
/// # Example
///
/// ```
/// use sorting_algorithm::merge_sort;
///
/// fn main() {
///     let mut data = [3, 1, 2, 5, 4];
///     
///     merge_sort::sort(&mut data);
///
///     assert_eq!(data, [1, 2, 3, 4, 5]);
/// }
/// ```
pub fn sort<T: Ord + Clone>(data: &mut [T]) {
    if data.len() > 1 {
        let mid = data.len() / 2;

        sort(&mut data[..mid]);
        sort(&mut data[mid..]);

        merge(data, mid);
    }
}

fn merge<T: Ord + Clone>(data: &mut [T], mid: usize) {
    let left = data[..mid].to_vec();
    let right = &data[mid..].to_vec();

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            data[k] = left[i].clone();
            i += 1
        } else {
            data[k] = right[j].clone();
            j += 1
        }

        k += 1;
    }

    while i < left.len() {
        data[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while j < right.len() {
        data[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}
