// arc1.rs
//
// 在这个练习中，我们有一个包含 u32 类型的 Vec，名为 "numbers"，其值范围从 0 到 99 -- [ 0, 1, 2, ..., 98, 99 ]
// 我们想要在 8 个不同的线程中同时使用这组数字。每个线程将计算每第八个数字的总和，但有一个偏移量。
//
// 第一个线程（偏移量 0），将计算 0, 8, 16, ... 的总和
// 第二个线程（偏移量 1），将计算 1, 9, 17, ... 的总和
// 第三个线程（偏移量 2），将计算 2, 10, 18, ... 的总和
// ...
// 第八个线程（偏移量 7），将计算 7, 15, 23, ... 的总和
//
// 因为我们使用线程，我们的值需要是线程安全的。因此，我们使用 Arc。
// 我们需要在两个 TODO 注释处进行修改。
//
// 通过在第一个 TODO 注释处填写 `shared_numbers` 的值，并在第二个 TODO 注释处创建 `child_numbers` 的初始绑定
// 来使这段代码能够编译。尽量不要创建 `numbers` Vec 的任何副本！
//
// 执行 `rustlings hint arc1` 或使用 `hint` watch 子命令来获取提示。

#![forbid(unused_imports)] // 不要更改这一行（或下一行）。
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers);
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let child_numbers = Arc::clone(&shared_numbers);
        joinhandles.push(thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
