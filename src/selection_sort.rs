pub fn sort<T: Ord>(data: &mut [T]) {
    let len = data.len();

    for i in 0..len {
        let mut min_index = i;

        for j in i + 1..len {
            if data[j] < data[min_index] {
                min_index = j;
            }
        }

        data.swap(i, min_index);
    }
}
