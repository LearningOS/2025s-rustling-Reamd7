// tests3.rs
//
// 此测试未测试我们的函数 - 使其以测试通过的方式执行此操作。
// 然后编写第二个测试，测试当我们调用 `is_even(5)` 时是否得到我们期望的结果。
//
// 执行 `rustlings hint tests3` 或使用 `hint` 观察子命令来获取
// 提示。

pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(2));
    }

    #[test]
    fn is_false_when_odd() {
        assert!(!is_even(3));
    }
}
