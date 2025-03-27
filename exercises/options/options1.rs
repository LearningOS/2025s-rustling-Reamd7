// options1.rs
//
// 执行 `rustlings hint options1` 或使用 `hint` watch 子命令来获取提示。

// 我还没有完成

// 这个函数返回冰箱里还剩多少冰淇淋。
// 如果在晚上10点之前，还剩5个。在晚上10点，有人把它们都吃完了，
// 所以就没有剩余的了 :(
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // 这里我们使用24小时制，所以晚上10点是22，凌晨12点是0
    // Option 输出应该优雅地处理 time_of_day > 23 的情况。
    // TODO: 完成函数体 - 记得返回一个 Option！
    if (time_of_day < 0 || time_of_day > 23) {
        None
    } else if time_of_day < 22 {
        Some(5)
    } else {
        Some(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: 修复这个测试。你如何获取 Option 中包含的值？
        let icecreams = maybe_icecream(12);
        assert_eq!(icecreams, Some(5));
    }
}
