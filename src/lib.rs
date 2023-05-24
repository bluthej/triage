mod bubblesort;
mod stdsort;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn std_sort_works() {
        let mut things = vec![4, 2, 3, 1];
        stdsort::sort(&mut things);
        assert_eq!(&things, &[1, 2, 3, 4]);
    }

    #[test]
    fn bubble_sort_works() {
        let mut things = vec![4, 2, 3, 1];
        bubblesort::sort(&mut things);
        assert_eq!(&things, &[1, 2, 3, 4]);
    }
}
