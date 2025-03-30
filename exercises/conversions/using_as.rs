// using_as.rs
//
// Rust 中的类型转换通过 `as` 运算符完成。请注意，
// `as` 运算符不仅在类型转换时使用。它还有助于
// 重命名导入。
//
// 目标是确保除法不会编译失败，并且
// 返回正确的类型。
//
// 执行 `rustlings hint using_as` 或使用 `hint` 观察子命令来获取
// 提示。
//

fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    total / values.len() as f64
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
