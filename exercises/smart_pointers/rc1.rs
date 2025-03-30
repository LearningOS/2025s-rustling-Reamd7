// rc1.rs
//
// 在这个练习中，我们想要通过 Rc<T> 类型来表达多重所有权的概念。
// 这是一个太阳系的模型 - 有一个 Sun 类型和多个 Planet。
// Planet 拥有 Sun 的所有权，表示它们围绕太阳运转。
//
// 通过使用适当的 Rc 原语来表达太阳有多个所有者，使这段代码能够编译。
//
// 执行 `rustlings hint rc1` 或使用 `hint` watch 子命令获取提示。

// 我还没有完成

use std::rc::Rc;

#[derive(Debug)]
struct Sun {}

#[derive(Debug)]
enum Planet {
    Mercury(Rc<Sun>),
    Venus(Rc<Sun>),
    Earth(Rc<Sun>),
    Mars(Rc<Sun>),
    Jupiter(Rc<Sun>),
    Saturn(Rc<Sun>),
    Uranus(Rc<Sun>),
    Neptune(Rc<Sun>),
}

impl Planet {
    fn details(&self) {
        println!("Hi from {:?}!", self)
    }
}

fn main() {
    let sun = Rc::new(Sun {});
    println!("reference count = {}", Rc::strong_count(&sun)); // 1 个引用

    let mercury = Planet::Mercury(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 2 个引用
    mercury.details();

    let venus = Planet::Venus(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 3 个引用
    venus.details();

    let earth = Planet::Earth(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 4 个引用
    earth.details();

    let mars = Planet::Mars(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 5 个引用
    mars.details();

    let jupiter = Planet::Jupiter(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 6 个引用
    jupiter.details();

    // 待完成
    let saturn = Planet::Saturn(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 7 个引用
    saturn.details();

    // 待完成
    let uranus = Planet::Uranus(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 8 个引用
    uranus.details();

    // 待完成
    let neptune = Planet::Neptune(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 9 个引用
    neptune.details();

    assert_eq!(Rc::strong_count(&sun), 9);

    drop(neptune);
    println!("reference count = {}", Rc::strong_count(&sun)); // 8 个引用

    drop(uranus);
    println!("reference count = {}", Rc::strong_count(&sun)); // 7 个引用

    drop(saturn);
    println!("reference count = {}", Rc::strong_count(&sun)); // 6 个引用

    drop(jupiter);
    println!("reference count = {}", Rc::strong_count(&sun)); // 5 个引用

    drop(mars);
    println!("reference count = {}", Rc::strong_count(&sun)); // 4 个引用

    drop(earth);
    // 待完成
    println!("reference count = {}", Rc::strong_count(&sun)); // 3 个引用

    drop(venus);
    // 待完成
    println!("reference count = {}", Rc::strong_count(&sun)); // 2 个引用

    drop(mercury);
    // 待完成
    println!("reference count = {}", Rc::strong_count(&sun)); // 1 个引用

    assert_eq!(Rc::strong_count(&sun), 1);
}
