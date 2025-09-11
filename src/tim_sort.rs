/*
TODO:
Implement galloping
Implement variable run size
Implement binary search merging for space efficiency
*/

use std::cmp::min;

use crate::insertion_sort;

const MIN_RUN: usize = 32;

/// Sorts a data set using Tim Sort
///
/// Average time complexity: O(n logn)
///
/// # Example
///
/// ```
/// use sorting_algorithm::tim_sort;
///
/// fn main() {
///     let mut data = [3, 1, 2, 5, 4];
///     
///     tim_sort::sort(&mut data);
///
///     assert_eq!(data, [1, 2, 3, 4, 5]);
/// }
/// ```
pub fn sort<T: Ord + Clone>(data: &mut [T]) {
    let n = data.len();

    if n <= 1 {
        return;
    }

    let mut start = 0;
    let mut run_stack: Vec<(usize, usize)> = Vec::new(); // (run_start, run_len)

    while start < n {
        let mut run_len = count_run(&mut data[start..]);

        if run_len < MIN_RUN {
            // extend run to min_run length
            let end = min(start + MIN_RUN, n);
            insertion_sort::sort(&mut data[start..end]);
            run_len = end - start;
        }

        run_stack.push((start, run_len));
        start += run_len;

        while run_stack.len() > 1 {
            let len = run_stack.len();
            let (top_start, top_len) = run_stack[len - 1];
            let (mid_start, mid_len) = run_stack[len - 2];
            let (bottom_start, bottom_len) = if len > 2 {
                run_stack[len - 3]
            } else {
                (usize::MAX, usize::MAX)
            };

            if len > 2 && bottom_len <= mid_len + top_len {
                if bottom_len < top_len {
                    merge(&mut data[bottom_start..mid_start + mid_len], mid_len);

                    run_stack[len - 3] = (bottom_start, bottom_len + mid_len);
                    run_stack.remove(len - 2);
                } else {
                    merge(&mut data[mid_start..top_start + top_len], top_len);
                    run_stack[len - 2] = (mid_start, mid_len + top_len);
                    run_stack.pop();
                }
            } else if mid_len <= top_len {
                merge(&mut data[mid_start..top_start + top_len], top_len);

                run_stack[len - 2] = (mid_start, mid_len + top_len);
                run_stack.pop();
            } else {
                break;
            }
        }
    }

    while run_stack.len() > 1 {
        let len = run_stack.len();
        let (top_start, top_len) = run_stack[len - 1];
        let (bottom_start, bottom_len) = run_stack[len - 2];

        merge(&mut data[bottom_start..top_start + top_len], bottom_len);

        run_stack[len - 2] = (bottom_start, bottom_len + top_len);
        run_stack.pop();
    }
}

fn merge<T: Ord + Clone>(data: &mut [T], run_split: usize) {
    let left = data[..run_split].to_vec();
    let right = data[run_split..].to_vec();

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
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

fn count_run<T: Ord>(data: &mut [T]) -> usize {
    let n = data.len();

    if n <= 1 {
        return 1;
    }

    let mut end = 1;

    if data[end] < data[0] {
        // descending run
        while end < n - 1 && data[end + 1] < data[end] {
            end += 1
        }

        data[..=end].reverse();
    } else {
        // ascending run
        while end < n - 1 && data[end + 1] >= data[end] {
            end += 1;
        }
    }

    end + 1 // add one to get length
}
