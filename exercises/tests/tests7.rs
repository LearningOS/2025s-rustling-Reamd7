// tests7.rs
//
// 在构建包时，某些依赖项既不能在 `Cargo.toml` 中导入，也不能直接链接；
// 某些预处理过程因代码生成而异，以设置特定于包的配置。
//
// Cargo 并不旨在取代其他构建工具，但它通过自定义构建脚本（称为 `build.rs`）与它们集成。
// 此文件通常放置在项目的根目录中，而在本例中，放置在本练习的同一目录中。
//
// 它可以用于：
//
// - 构建捆绑的 C 库。
// - 在主机系统上查找 C 库。
// - 从规范生成 Rust 模块。
// - 执行 crate 所需的任何平台特定配置。
//
// 在设置配置时，我们可以在构建脚本中使用 `println!` 来告诉 Cargo 遵循一些指令。
// 通用格式为：
//
//     println!("cargo:{}", your_command_in_string);
//
// 有关构建脚本的更多信息，请参阅官方 Cargo 手册：
// https://doc.rust-lang.org/cargo/reference/build-scripts.html
//
// 在本练习中，我们查找一个环境变量，并期望它落在一个范围内。您可以查看测试用例以了解详细信息。
//
// 您不应修改此文件。修改同一目录中的 `build.rs` 以通过此练习。
//
// 执行 `rustlings hint tests7` 或使用 `hint` 观察子命令来获取
// 提示。

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let s = std::env::var("TEST_FOO").unwrap();
        let e: u64 = s.parse().unwrap();
        assert!(timestamp >= e && timestamp < e + 10);
    }
}
