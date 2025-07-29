# sorting-algorithm

various sorting algorithms implemented in Rust

The current implemented sorting algorithms are:

- Bogo Sort
- Bubble Sort
- Cocktail Shaker Sort
- Comb Sort
- Gnome Sort
- Insertion Sort
- Merge Sort
- Quick Sort
- Selection Sort
- Shell Sort

## Use

The use for each algorithm is essentially the same.

1. Import the module for the desired algorithm
2. Call the module's `sort()` function
3. Pass a mutable pointer as an argument

```Rust
use sorting_algorithm::bubble_sort;

fn main() {
    let &mut data = [3, 1, 2, 5, 4];

    bubble_sort::sort(&mut data);

    assert_eq!(data, [1, 2, 3, 4, 5]);
}
```
