// try_from_into.rs
//
// TryFrom 是一个简单且安全的类型转换，在某些情况下可能会以受控方式失败。
// 基本上，这与 From 相同。主要区别在于这应该返回一个 Result 类型，
// 而不是目标类型本身。您可以在
// https://doc.rust-lang.org/std/convert/trait.TryFrom.html 阅读更多相关信息
//
// 执行 `rustlings hint try_from_into` 或使用 `hint` 观察子命令来获取
// 提示。

use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// 我们将使用此错误类型来实现这些 `TryFrom` 转换。
#[derive(Debug, PartialEq)]
enum IntoColorError {
    // 切片长度不正确
    BadLen,
    // 整数转换错误
    IntConversion,
}

// 您的任务是完成此实现并返回内部类型 Color 的 Ok 结果。
// 您需要为三个整数的元组、三个整数的数组和一个整数切片创建一个实现。
//
// 请注意，元组和数组的实现将在编译时进行检查，但切片实现需要检查切片长度！
// 另请注意，正确的 RGB 颜色值必须是 0..=255 范围内的整数。

fn check_u8(value: i16) -> Result<u8, IntoColorError> {
    if value < 0 || value > 255 {
        Err(IntoColorError::IntConversion)
    } else {
        Ok(value as u8)
    }
}

fn check_color(tuple: (i16, i16, i16)) -> Result<Color, IntoColorError> {
    let (red, green, blue) = tuple;
    if red > 255 || green > 255 || blue > 255 {
        Err(IntoColorError::IntConversion)
    } else {
        if red < 0 || green < 0 || blue < 0 {
            Err(IntoColorError::IntConversion)
        } else {
            let red = check_u8(red)?;
            let green = check_u8(green)?;
            let blue = check_u8(blue)?;
            Ok(Color { red, green, blue })
        }
    }
}

// 元组实现
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;
    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        check_color(tuple)
    }
}

// 数组实现
impl TryFrom<[i16; 3]> for Color {
    type Error = IntoColorError;
    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        check_color((arr[0], arr[1], arr[2]))
    }
}

// 切片实现
impl TryFrom<&[i16]> for Color {
    type Error = IntoColorError;
    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        if slice.len() != 3 {
            Err(IntoColorError::BadLen)
        } else {
            check_color((slice[0], slice[1], slice[2]))
        }
    }
}

fn main() {
    // 使用 `try_from` 函数
    let c1 = Color::try_from((183, 65, 14));
    println!("{:?}", c1);

    // 由于 TryFrom 已为 Color 实现，我们应该能够使用 TryInto
    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{:?}", c2);

    let v = vec![183, 65, 14];
    // 对于切片，我们应该使用 `try_from` 函数
    let c3 = Color::try_from(&v[..]);
    println!("{:?}", c3);
    // 或者在圆括号内取切片并使用 TryInto
    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{:?}", c4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_out_of_range_positive() {
        assert_eq!(
            Color::try_from((256, 1000, 10000)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_out_of_range_negative() {
        assert_eq!(
            Color::try_from((-1, -10, -256)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_sum() {
        assert_eq!(
            Color::try_from((-1, 255, 255)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_correct() {
        let c: Result<Color, _> = (183, 65, 14).try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_array_out_of_range_positive() {
        let c: Result<Color, _> = [1000, 10000, 256].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_out_of_range_negative() {
        let c: Result<Color, _> = [-10, -256, -1].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_sum() {
        let c: Result<Color, _> = [-1, 255, 255].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_correct() {
        let c: Result<Color, _> = [183, 65, 14].try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_slice_out_of_range_positive() {
        let arr = [10000, 256, 1000];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_out_of_range_negative() {
        let arr = [-256, -1, -10];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_sum() {
        let arr = [-1, 255, 255];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_correct() {
        let v = vec![183, 65, 14];
        let c: Result<Color, _> = Color::try_from(&v[..]);
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_slice_excess_length() {
        let v = vec![0, 0, 0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(IntoColorError::BadLen));
    }
    #[test]
    fn test_slice_insufficient_length() {
        let v = vec![0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(IntoColorError::BadLen));
    }
}
