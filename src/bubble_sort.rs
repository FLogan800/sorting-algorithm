pub fn sort<T: PartialOrd>(data: &mut [T]) {
    let len = data.len();

    for i in 0..len {
        let mut swapped = false;

        for j in 0..(len - i - 1) {
            if data[j] > data[j + 1] {
                data.swap(j, j + 1);
                swapped = true;
            }
        }

        if !swapped {
            return;
        }
    }
}
