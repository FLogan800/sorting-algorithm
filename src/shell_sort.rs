pub fn sort<T: Ord>(data: &mut [T]) {
    let n = data.len();

    let mut gap = n / 2;

    while gap > 0 {
        for i in gap..n {
            let mut j = i;

            while j >= gap && data[j] < data[j - gap] {
                data.swap(j, j - gap);
                j -= gap;
            }
        }

        gap /= 2;
    }
}
