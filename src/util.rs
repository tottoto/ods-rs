pub(crate) fn allocate<T>(length: usize) -> Box<[Option<T>]> {
    std::iter::repeat_with(|| None).take(length).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocate_successful() {
        let a = allocate::<u32>(3);

        assert_eq!(a.len(), 3);

        for v in a {
            assert_eq!(v, None);
        }
    }
}
