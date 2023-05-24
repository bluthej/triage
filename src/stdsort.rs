#[allow(unused)]
pub fn sort<T: Ord>(slice: &mut [T]) {
    slice.sort();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut things = vec![4, 2, 3, 1];
        sort(&mut things);
        assert_eq!(&things, &[1, 2, 3, 4]);
    }
}
