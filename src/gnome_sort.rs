pub fn sort<T: PartialOrd>(data: &mut [T]) {
    let len = data.len();

    let mut i = 0;

    while i < len {
        if i == 0 {
            i += 1;
        }

        if data[i] >= data[i - 1] {
            i += 1;
        } else {
            data.swap(i, i - 1);
            i -= 1;
        }
    }
}
