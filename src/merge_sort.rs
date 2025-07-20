/// Sorts a data set using Merge Sort
///
/// Average time complexity: O(n*n!)
///
/// # Examples
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
    let right = data[mid..].to_vec();

    let mut i = 0;
    let mut j = 0;

    for element in data {
        if i < left.len() && j < right.len() {
            if left[i] < right[j] {
                *element = left[i].clone();
                i += 1;
            } else {
                *element = right[j].clone();
                j += 1;
            }
        } else if i < left.len() {
            *element = left[i].clone();
            i += 1;
        } else if j < right.len() {
            *element = right[j].clone();
            j += 1;
        }
    }
}
