// tests6.rs
//
// 在此示例中，我们浅入 Rust 标准库的 unsafe 函数。修复所有问号和 todos 以使测试通过。
//
// 执行 `rustlings hint tests6` 或使用 `hint` 观察子命令来获取
// 提示。


struct Foo {
    a: u128,
    b: Option<String>,
}

/// # Safety
///
/// `ptr` 必须包含 `Foo` 的拥有的 box。
unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {
    // SAFETY: 根据合同，`ptr` 包含 `Foo` 的拥有的 box。我们只需从该指针重建 box。
    let mut ret: Box<Foo> = unsafe { std::boxed::Box::from_raw(ptr) };
    ret.b = Some("hello".to_owned());
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_success() {
        let data = Box::new(Foo { a: 1, b: None });

        let ptr_1 = &data.a as *const u128 as usize;
        // SAFETY: 我们传递 `Foo` 的拥有的 box。
        let ret = unsafe { raw_pointer_to_box(Box::into_raw(data)) };

        let ptr_2 = &ret.a as *const u128 as usize;

        assert!(ptr_1 == ptr_2);
        assert!(ret.b == Some("hello".to_owned()));
    }
}
