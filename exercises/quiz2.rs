// quiz2.rs
//
// 这是一个关于以下章节的测验：
// - 字符串
// - 向量
// - 移动语义
// - 模块
// - 枚举
//
// 让我们构建一个函数形式的机器。作为输入，我们将给出一个字符串和命令的列表。
// 这些命令决定了要对字符串执行什么操作。可以是：
// - 将字符串转换为大写
// - 修剪字符串
// - 将"bar"追加到字符串指定次数
// 具体形式如下：
// - 输入将是一个长度为2的元组的向量，
//   第一个元素是字符串，第二个是命令。
// - 输出元素将是一个字符串向量。
//
// 这次没有提示！

// 我还没有完成

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: 完成函数签名！
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // TODO: 完成输出声明！
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            // TODO: 完成函数体。你可以做到的！
            match command {
                Command::Uppercase => output.push(string.to_uppercase()),
                Command::Trim => output.push(string.trim().to_string()),
                Command::Append(n) => output.push(format!("{}{}", string, "bar".repeat(*n))),
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: 我们需要导入什么来使 `transformer` 在作用域内？
    use crate::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
