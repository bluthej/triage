#[allow(unused)]
pub fn sort<T: Ord>(slice: &mut [T]) {
    let mut has_swapped = true;
    while has_swapped {
        has_swapped = false;
        for i in 1..slice.len() {
            if slice[i - 1] > slice[i] {
                slice.swap(i - 1, i);
                has_swapped = true;
            }
        }
    }
}
