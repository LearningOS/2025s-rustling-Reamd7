// from_into.rs
//
// From trait 用于 value-to-value 的转换。如果 From 对于一个类型实现了
// 正确的实现，那么 Into trait 应该反过来工作。你可以在
// https://doc.rust-lang.org/std/convert/trait.From.html 阅读更多关于它的信息
//
// 执行 `rustlings hint from_into` 或使用 `hint` 观察子命令来获取
// 提示。

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// 我们实现 Default trait 以便在
// 提供的字符串无法转换为 Person 对象时使用它作为后备
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

// 你的任务是完成这个实现，以便 `let p = Person::from("Mark,20")`
// 这一行能够编译。请注意，你需要使用类似 `"4".parse::<usize>()` 的方法
// 将 age 组件解析为 `usize`。这个操作的结果需要被适当地处理。
//
// 步骤：
// 1. 如果提供的字符串的长度为 0，则返回 Person 的默认值。
// 2. 在字符串中存在的逗号上分割给定的字符串。
// 3. 从分割操作中提取第一个元素，并将其用作名称。
// 4. 如果名称为空，则返回 Person 的默认值。
// 5. 从分割操作中提取另一个元素，并将其解析为 `usize` 作为年龄。
// 如果在解析年龄时出现问题，则返回 Person 的默认值。
// 否则，返回一个使用结果实例化的 Person 对象。

impl From<&str> for Person {
    fn from(s: &str) -> Person {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Person::default();
        }
        let name = parts[0].to_string();
        if name.is_empty() {
            return Person::default();
        }
        let age = parts[1].parse::<usize>();
        if age.is_err() {
            return Person::default();
        }
        Person { name, age: age.unwrap() }
    }
}

fn main() {
    // 使用 `from` 函数
    let p1 = Person::from("Mark,20");
    // 由于 From 已经为 Person 实现，我们应该能够使用 Into
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        // 测试默认的人是 30 岁的 John
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }
    #[test]
    fn test_bad_convert() {
        // 测试当提供错误的字符串时，返回 John
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_good_convert() {
        // 测试 "Mark,20" 是否有效
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    #[test]
    fn test_bad_age() {
        // 测试 "Mark,twenty" 是否会由于解析年龄时出错而返回默认人
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,man");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
