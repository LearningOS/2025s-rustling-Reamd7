// tests8.rs
//
// 此练习与之前的练习共享 `build.rs`。
// 您需要在 `build.rs` 中添加一些代码，以使此练习和之前的练习都能正常工作。
//
// 执行 `rustlings hint tests8` 或使用 `hint` 观察子命令来获取
// 提示。

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        #[cfg(feature = "pass")]
        return;

        panic!("no cfg set");
    }
}
