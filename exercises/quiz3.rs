// quiz3.rs
//
// 这个测验测试：
// - 泛型
// - 特征
//
// 一所想象中的魔法学校有一个新的成绩单生成系统，使用 Rust 编写！
// 目前该系统仅支持创建以数字表示学生成绩的成绩单（例如 1.0 -> 5.5）。
// 但是，学校也会发布字母等级的成绩（A+ -> F-），并且需要能够打印这两种类型的成绩单！
//
// 在 ReportCard 结构体和 impl 块中进行必要的代码更改，以支持字母等级的成绩单。
// 在第二个测试中将 Grade 改为 "A+" 以表明你的更改允许使用字母等级。
//
// 执行 `rustlings hint quiz3` 或使用 `hint` watch 子命令获取提示。

use std::fmt::Display;
pub struct ReportCard<T: ToString> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

impl<T: ToString + Display> ReportCard<T> {
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
