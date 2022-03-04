pub(crate) fn idx_after(start: usize, haystack: &str, needle: char) -> Option<usize> {
    haystack[start..].find(needle).map(|i| i + start)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idx_after() {
        assert_eq!(idx_after(0, "abcdefgittmm", 'm'), Some(10));
        assert_eq!(idx_after(0, "abc", 'd'), None);
        assert_eq!(idx_after(1, "abc", 'b'), Some(1));
        assert_eq!(idx_after(1, "abc", 'd'), None);
        assert_eq!(idx_after(2, "abc", 'b'), None);
    }

}