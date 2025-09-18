use crate::insertion_sort;
use std::{cmp::min, f32::consts::LN_10};

const MIN_MERGE: usize = 32;
const MIN_GALLOP: i8 = 7;

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

    if n < 2 {
        return;
    }

    if n < MIN_MERGE {
        insertion_sort::sort(data);
        return;
    }

    let mut run_stack: Vec<(usize, usize)> = Vec::new(); // (start, len)

    let min_run = min_run_length(n);
    let mut low = 0;

    while low < n {
        let mut run_len = count_run_and_make_ascending(&mut data[low..]);

        if run_len < min_run {
            let target_len = min(low + min_run, n);
            insertion_sort::sort(&mut data[low..target_len]);
            run_len = target_len;
        }

        run_stack.push((low, run_len));
        merge_collapse(data, &mut run_stack);

        low += run_len
    }

    merge_force_collapse(&mut data, &mut run_stack);
}

fn count_run_and_make_ascending<T: Ord + Clone>(data: &mut [T]) -> usize {
    let mut run_hi = 2;

    if run_hi == data.len() {
        return 1;
    }

    if data[1] < data[0] {
        // Descending
        while run_hi < data.len() && data[run_hi] < data[run_hi - 1] {
            run_hi += 1;
        }

        data[..run_hi].reverse();
    } else {
        // Ascending
        while run_hi < data.len() && data[run_hi] >= data[run_hi - 1] {
            run_hi += 1;
        }
    }

    return run_hi;
}

fn min_run_length(n: usize) -> usize {
    let mut cur_n = n;
    let mut r = 0;
    while cur_n >= MIN_MERGE {
        r |= cur_n & 1; // Checks if n is odd
        cur_n >>= 1; // Divides cur_n by 2
    }

    cur_n + r
}

fn merge_collapse<T: Ord + Clone>(data: &mut [T], run_stack: &mut Vec<(usize, usize)>) {
    let mut len = run_stack.len();

    while len > 1 {
        let mut n = len - 2;

        if n > 0 && run_stack[n - 1].1 <= run_stack[n].1 + run_stack[n + 1].1
            || n > 1 && run_stack[n - 2].1 <= run_stack[n].1 + run_stack[n - 1]
        {
            if run_stack[n - 1].1 < run_stack[n + 1].1 {
                n -= 1;
            } else if n < 0 || run_stack[n].1 > run_stack[n + 1].1 {
                break;
            }
        }

        merge_at(data, run_stack, n);
    }
}

fn merge_force_collapse() {}

fn merge_at<T: Ord + Clone>(data: &mut [T], run_stack: &mut Vec<(usize, usize)>, i: usize) {
    let (mut base1, mut len1) = run_stack[i];
    let (mut base2, mut len2) = run_stack[i+1];

    run_stack[i].1 = len1 + len2;
    if run_stack.len() - i == 3 {
        run_stack[i+1] = run_stack[i+2];
    }
    run_stack.pop();

    let k = gallop_right();

    base1 += k;
    len1 -= k;

    if len1 == 0 {
        return;
    }

    len2 = gallop_left();

    if len2 == 0 {
        return;
    }

    if len1 <= len2 {
        merge_low();
    } else {
        merge_high();
    }
}

fn gallop_left<T: Ord>(key: T, data: &mut [T], hint: usize) -> usize {}

fn gallop_right() -> usize {}

fn merge_low() {}

fn merge_high() {}
