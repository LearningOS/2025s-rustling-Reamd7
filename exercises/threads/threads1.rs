// threads1.rs
//
// 这个程序会生成多个线程，每个线程至少运行 250 毫秒，并且
// 每个线程都会返回它们完成所花费的时间。程序应该
// 等待所有生成的线程完成，并将它们的
// 返回值收集到一个 vector 中。
//
// 执行 `rustlings hint threads1` 或使用 `hint` watch 子命令来获取
// 提示。

// 我还没有完成

use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
            start.elapsed().as_millis()
        }));
    }

    let mut results: Vec<u128> = vec![];
    for handle in handles {
        let result = handle.join().unwrap();
        results.push(result);
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}
