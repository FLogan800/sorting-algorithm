use fastrand;

pub fn sort<T: Ord>(data: &mut [T]) {
    while !data.is_sorted() {
        fastrand::shuffle(data);
    }
}
