#[allow(unused)]
pub fn sort<T: Ord>(slice: &mut [T]) {
    for unsorted in 0..slice.len() {
        let mut imin = unsorted;
        for (i, v) in slice[(unsorted + 1)..].iter().enumerate() {
            if v < &slice[imin] {
                imin = unsorted + 1 + i;
            }
        }
        slice.swap(unsorted, imin);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut things = vec![4, 2, 3, 1, 5];
        sort(&mut things);
        assert_eq!(&things, &[1, 2, 3, 4, 5]);
    }
}
