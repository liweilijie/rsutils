// haystack从start开始一直到最后, 查找needle的下标,下标相对于start的位置从0开始的
// 如果没有找到, 返回None
// 如果找到, 返回Some(下标) 此处的下标就是haystack原始字符串中的下标
pub(crate) fn idx_after(start: usize, haystack: &str, needle: char) -> Option<usize> {
    haystack[start..].find(needle).map(|i| i + start)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idx_after() {
        assert_eq!(idx_after(0, "hello world", 'w'), Some(6));
        assert_eq!(idx_after(0, "have a good night", 'o'), Some(8));
        assert_eq!(idx_after(1, "abc", 'b'), Some(1));
        assert_eq!(idx_after(1, "abc", 'd'), None);
    }
}