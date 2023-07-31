#[allow(unused)]
pub fn sort<T: Ord>(slice: &mut [T]) {
    for unsorted in 0..slice.len() {
        let mut index = unsorted;
        for i in (unsorted + 1)..slice.len() {
            if slice[i] < slice[index] {
                index = i;
            }
        }
        slice.swap(unsorted, index);
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
