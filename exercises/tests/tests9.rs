// tests9.rs
//
// Rust 非常能够与 C/C++ 和其他静态编译语言共享 FFI 接口，它甚至可以在代码本身中链接！
// 它通过 extern 块进行，就像下面的代码一样。
//
// `extern` 关键字后面的短字符串指示外部导入的函数将遵循哪个 ABI。
// 在本练习中，使用 "Rust"，而其他变体存在，例如 "C" 用于标准 C ABI，"stdcall" 用于 Windows ABI。
//
// 外部导入的函数在 extern 块中声明，用分号标记签名结尾，而不是用大括号。
// 某些属性可以应用于这些函数声明以修改链接行为，例如 #[link_name = ".."] 以修改实际符号名称。
//
// 如果您想将您的符号导出到链接环境，则 `extern` 关键字也可以在具有相同 ABI 字符串注释的函数定义之前标记。
// Rust 函数的默认 ABI 实际上是 "Rust"，因此如果您想链接到纯 Rust 函数，则可以省略整个 extern 术语。
//
// 默认情况下，Rust 会修改符号，就像 C++ 一样。要禁止此行为并使这些函数可以通过名称寻址，可以应用属性 #[no_mangle]。
//
// 在本练习中，您的任务是使测试用例能够调用模块 Foo 中的 `my_demo_function`。
// `my_demo_function_alias` 是 `my_demo_function` 的别名，因此测试用例中的两行代码应该调用同一个函数。
//
// 除了添加两行属性之外，您不应修改任何现有代码。

extern "Rust" {
    fn my_demo_function(a: u32) -> u32;
    #[link_name = "my_demo_function"]
    fn my_demo_function_alias(a: u32) -> u32;
}

mod Foo {
    // 没有 `extern` 等于 `extern "Rust"`。
    #[no_mangle]
    fn my_demo_function(a: u32) -> u32 {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // 外部导入的函数默认情况下是 UNSAFE 的，因为来自其他语言的来源不受信任。
        // 您可以将它们包装在安全的 Rust API 中，以减轻调用者的负担。
        //
        // SAFETY: 我们知道这些函数是安全 Rust 函数的别名。
        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}
