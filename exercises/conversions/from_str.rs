// from_str.rs
//
// 这与 from_into.rs 类似，但这次我们将实现 `FromStr`
// 并返回错误而不是回退到默认值。此外，在
// 实现 FromStr 后，您可以使用字符串上的 `parse` 方法来生成
// 实现器类型的对象。您可以在
// https://doc.rust-lang.org/std/str/trait.FromStr.html 阅读更多相关信息
//
// 执行 `rustlings hint from_str` 或使用 `hint` 观察子命令来获取
// 提示。

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// 我们将使用此错误类型来实现 `FromStr`。
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // 空输入字符串
    Empty,
    // 不正确的字段数
    BadLen,
    // 空名称字段
    NoName,
    // 来自 parse::<usize>() 的包装错误
    ParseInt(ParseIntError),
}

// 步骤：
// 1. 如果提供的字符串的长度为 0，则应返回错误
// 2. 在字符串中存在的逗号上分割给定的字符串
// 3. 应该只从分割返回 2 个元素，否则返回一个错误
// 4. 从分割操作中提取第一个元素，并将其用作名称
// 5. 从分割操作中提取另一个元素，并使用类似 `"4".parse::<usize>()` 的方法将其解析为 `usize` 作为年龄
// 6. 如果在提取名称和年龄时出现问题，则应返回一个错误
// 如果一切顺利，则返回 Person 对象的 Result

// 顺便说一句：`Box<dyn Error>` 实现了 `From<&'_ str>`。这意味着如果您想返回一个字符串错误消息，您可以通过只使用 return `Err("my error message".into())` 来实现。

impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        if s.is_empty() {
            return Err(ParsePersonError::Empty);
        }
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Err(ParsePersonError::BadLen);
        }
        let name = parts[0].to_string();
        if name.is_empty() {
            return Err(ParsePersonError::NoName);
        }
        let age = parts[1].parse::<usize>();
        if age.is_err() {
            return Err(ParsePersonError::ParseInt(age.err().unwrap()));
        }
        Ok(Person { name, age: age.unwrap() })
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    fn missing_age() {
        assert!(matches!(
            "John,".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(
            "John,twenty".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(
            ",".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!(
            "John,32,man".parse::<Person>(),
            Err(ParsePersonError::BadLen)
        );
    }
}
