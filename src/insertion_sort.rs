pub fn sort<T: Ord>(data: &mut [T]) {
    for i in 1..data.len() {
        let mut j = i;

        while j > 0 && data[j] < data[j - 1] {
            data.swap(j, j - 1);
            j -= 1;
        }
    }
}
