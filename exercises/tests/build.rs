//! 这是 tests7 和 tests8 的构建脚本。
//!
//! 您应该修改此文件以使两个练习都通过。

fn main() {
    // 在 tests7 中，我们应该设置一个环境变量
    // 称为 `TEST_FOO`。在标准输出中打印以让
    // Cargo 执行它。
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // 此处使用此时间戳有什么用？
    let your_command = format!(
        "Your command here with {}, please checkout exercises/tests/build.rs",
        timestamp
    );
    println!("cargo:{}", your_command);

    // 在 tests8 中，我们应该启用 "pass" 功能以使
    // 测试用例提前返回。填写命令以告知
    // Cargo 关于这一点。
    let your_command = "Your command here, please checkout exercises/tests/build.rs";
    println!("cargo:{}", your_command);
}
