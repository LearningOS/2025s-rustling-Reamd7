// tests5.rs
//
// Rust 中的 `unsafe` 充当合同。
//
// 当 `unsafe` 标记在项目声明上时，例如函数、
// trait 等，它会声明一个合同。但是，
// 合同的内容不能仅通过单个关键字来表达。
// 因此，您有责任在 `# Safety` 中手动声明它
// 项目的文档注释部分。
//
// 当 `unsafe` 标记在由大括号括起来的代码块上时，
// 它声明遵守某些合同，例如某些
// 指针参数的有效性，某些内存地址的所有权。但是，像
// 上面的文字一样，您仍然需要在
// 代码块上的注释中说明如何遵守合同。
//
// 注意：所有注释都用于代码的可读性和可维护性，而 Rust 编译器将代码的健全性信任给您自己！如果您无法证明您自己的代码的内存安全性和健全性，请退一步并使用安全代码！
//
// 执行 `rustlings hint tests5` 或使用 `hint` 观察子命令来获取
// 提示。

/// # Safety
///
/// `address` 必须包含对有效 `u32` 值的可变引用。
unsafe fn modify_by_address(address: usize) {
    // TODO: 填写下面代码块的安全声明，以匹配您的代码行为和此函数的合同。您可以使用下面测试的注释作为格式参考。
    unsafe {
        // SAFETY: 我们知道地址是有效的，并且包含对 `u32` 局部变量的唯一引用。
        let ptr = address as *mut u32;
        *ptr = 0xAABBCCDD;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: 保证地址有效，并且包含对 `u32` 局部变量的唯一引用。
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert!(t == 0xAABBCCDD);
    }
}
