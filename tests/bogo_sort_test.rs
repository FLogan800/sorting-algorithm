use sorting_algorithm::bogo_sort::sort;

#[test]
fn random_data() {
    let mut data = [5, 9, 2, 3, 1, 4, 8, 0, 7, 6];

    sort(&mut data);

    assert_eq!(data, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn reverse_data() {
    let mut data = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];

    sort(&mut data);

    assert_eq!(data, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn repeated_data() {
    let mut data = [3, 1, 0, 4, 4, 2, 1, 2, 3, 0];

    sort(&mut data);

    assert_eq!(data, [0, 0, 1, 1, 2, 2, 3, 3, 4, 4]);
}

#[test]
fn one_number() {
    let mut data = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1];

    sort(&mut data);

    assert_eq!(data, [1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn empty_array() {
    let mut data: [i32; 0] = [];

    sort(&mut data);

    assert_eq!(data, []);
}

#[test]
fn almost_sorted() {
    let mut data = [1, 0, 2, 3, 4, 5, 6, 7, 8, 9];

    sort(&mut data);

    assert_eq!(data, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}
