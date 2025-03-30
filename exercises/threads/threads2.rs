// threads2.rs
//
// 在上一个练习的基础上，我们希望所有的线程都完成它们的工作，
// 但这一次，生成的线程需要负责更新一个
// 共享值：JobStatus.jobs_completed
//
// 执行 `rustlings hint threads2` 或使用 `hint` watch 子命令来获取
// 提示。



use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            let mut status_shared = status_shared.lock().unwrap();
            // TODO: You must take an action before you update a shared value
            status_shared.jobs_completed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice
        // anything interesting in the output? Do you have to 'join' on all the
        // handles?
        println!("jobs completed {}", status.lock().unwrap().jobs_completed);
    }
}
