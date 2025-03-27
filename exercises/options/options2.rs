// options2.rs
//
// 执行 `rustlings hint options2` 或使用 `hint` watch 子命令来获取提示。


#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: 将其改为 if let 语句，其值为 "Some" 类型
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: 将其改为 while let 语句 - 记住 vector.pop() 也会
        // 添加另一层 Option<T>。你可以将 `Option<T>` 嵌套在
        // while let 和 if let 中。
        while let Some(integer) = optional_integers.pop() {
            if let Some(integer) = integer {
                assert_eq!(integer, cursor);
                cursor -= 1;
            }
        }

        assert_eq!(cursor, 0);
    }
}
