use fastrand;

pub fn sort<T: PartialOrd>(data: &mut [T]) {
    if data.len() > 1 {
        let pivot = partition(data);

        sort(&mut data[..pivot]);
        sort(&mut data[pivot + 1..]);
    }
}

fn partition<T: PartialOrd>(data: &mut [T]) -> usize {
    let n = data.len();
    let pivot = fastrand::usize(0..data.len());
    data.swap(pivot, n - 1);

    let mut i = 0;

    for j in 0..n - 1 {
        if data[j] <= data[n - 1] {
            data.swap(i, j);
            i += 1;
        }
    }

    data.swap(i, n - 1);
    i
}
