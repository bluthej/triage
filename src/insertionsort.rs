#[allow(unused)]
pub fn sort<T: Ord>(slice: &mut [T]) {
    for unsorted in 1..slice.len() {
        let i = slice[..unsorted]
            .binary_search(&slice[unsorted])
            .unwrap_or_else(std::convert::identity);
        slice[i..=unsorted].rotate_right(1);
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
