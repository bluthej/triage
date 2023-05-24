#[allow(unused)]
pub fn sort<T: Ord>(slice: &mut [T]) {
    for unsorted in 1..slice.len() {
        let mut i = unsorted;
        while i > 0 && slice[i - 1] > slice[i] {
            slice.swap(i - 1, i);
            i -= 1;
        }
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
