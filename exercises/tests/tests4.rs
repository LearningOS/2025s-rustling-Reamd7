// tests4.rs
//
// 确保我们正在测试正确的条件！
//
// 执行 `rustlings hint tests4` 或使用 `hint` 观察子命令来获取
// 提示。

struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    // 仅更改测试函数本身
    pub fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("Rectangle width and height cannot be negative!")
        }
        Rectangle {width, height}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // 此测试应检查矩形是否是我们传递给其构造函数的大小
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10); // 检查宽度
        assert_eq!(rect.height, 20); // 检查高度
    }

    #[test]
    #[should_panic]
    fn negative_width() {
        // 此测试应检查当我们尝试创建具有负宽度的矩形时程序是否会发生 panic
        let _rect = Rectangle::new(-10, 10);
    }

    #[test]
    #[should_panic]
    fn negative_height() {
        // 此测试应检查当我们尝试创建具有负高度的矩形时程序是否会发生 panic
        let _rect = Rectangle::new(10, -10);
    }
}
