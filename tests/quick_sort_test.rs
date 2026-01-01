use sorting_algorithm::quick_sort::sort;
use fastrand;

#[test]
fn test_random_data() {
    for n in [1, 10, 100, 1_000] { 
        let mut data: Vec<u32> = (0..n as u32).collect();
        fastrand::shuffle(&mut data);

        let mut expected = data.clone();
        expected.sort();

        sort(&mut data);

        assert_eq!(data, expected);
    }
}

#[test]
fn test_sorted_data() {
    for n in [1, 10, 100, 1_000] {
        let mut data: Vec<u32> = (0..n).collect();

        let expected = data.clone();

        sort(&mut data);

        assert_eq!(data, expected);
    }
}

#[test]
fn test_reversed_data() {
    for n in [1, 10, 100, 1_000] {
        let mut data: Vec<u32> = (0..n).rev().collect();

        let mut expected = data.clone();
        expected.sort();

        sort(&mut data);

        assert_eq!(data, expected);
    }
}

#[test]
fn test_empty_data() {
    let mut data: [u32; 0] = [];

    sort(&mut data);

    assert_eq!(data, []);
}

#[test]
fn test_repeated_number() {
    for n in [1, 10, 100, 1_000] {
        let mut data: Vec<u32> = Vec::new();

        for _ in 0..n {
            data.push(0);
        }

        let mut expected = data.clone();
        expected.sort();

        sort(&mut data);

        assert_eq!(data, expected);
    }
}