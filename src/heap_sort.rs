/// Sorts a data set using Heap Sort
///
/// Average time complexity: O(n logn)
///
/// # Examples
///
/// ```
/// use sorting_algorithm::heap_sort;
///
/// fn main() {
///     let mut data = [3, 1, 2, 5, 4];
///     
///     heap_sort::sort(&mut data);
///
///     assert_eq!(data, [1, 2, 3, 4, 5]);
/// }
/// ```
pub fn sort<T: Ord>(data: &mut [T]) {
    let n = data.len();

    if n <= 1 {
        return;
    }

    for i in (0..n / 2).rev() {
        heapify(data, i);
    }

    for i in (1..n).rev() {
        data.swap(0, i);

        heapify(&mut data[..i], 0);
    }
}

fn heapify<T: Ord>(data: &mut [T], root: usize) {
    let n = data.len();

    let mut largest = root;

    let left = 2 * root + 1;
    let right = 2 * root + 2;

    if left < n && data[left] > data[largest] {
        largest = left;
    }

    if right < n && data[right] > data[largest] {
        largest = right;
    }

    if largest != root {
        data.swap(root, largest);

        heapify(data, largest);
    }
}
