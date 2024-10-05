// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.


use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::sync::Mutex;

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
            // TODO: You must take an action before you update a shared value
            status_shared.lock().unwrap().jobs_completed += 1;
        });//应该会自动drop掉锁
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice
        // anything interesting in the output? Do you have to 'join' on all the
        // handles?
        // 使用了Mutex,主线程要想读也得申请一个锁，因此应该不用join了
        // 这里join是为了让所有子线程都结束吧
        // 注释掉join后，所有数值都会输出0，主线程要了锁锁住了变量
        // println!("jobs completed {}", status.lock().unwrap().jobs_completed);输出不固定
        println!("jobs completed {}", status.lock().unwrap().jobs_completed);
    }
}
