fn compute(a: u32, b: u32) -> u32 {
    // TODO: change the line below to fix the compiler error and make the tests pass.
    // 这儿如果不写 u32 会根据上下文进行推断。也可以判断出是 u32，如果没有任何上下文。数字比如 444  也默认类型 u32
    // 下面也可以写  a + b * 4
    a + b * 4u32
}

#[cfg(test)]
mod tests {
    use crate::compute;

    #[test]
    fn case() {
        assert_eq!(compute(1, 2), 9);
    }
}
