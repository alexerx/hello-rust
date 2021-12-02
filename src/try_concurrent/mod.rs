use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Mutex, Arc};

pub fn run() {
  // try_spawn();
  // try_channel();
  try_mutex();
}

pub fn try_spawn() {

  // 返回JoinHandle句柄
  let handle = thread::spawn(|| {
    for i in 1..10 {
      println!("hi number {} from the spawned thread!", i);
      thread::sleep(Duration::from_millis(1));
    }
  });

  handle.join().unwrap();

  for i in 1..5 {
    println!("hi number {} from the main thread!", i);
    thread::sleep(Duration::from_millis(1));
  }

  // join方法等待线程执行完
  // handle.join().unwrap();
}

pub fn try_channel() {
  // mpsc: multiple producer, single consumer
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let val = String::from("hi");
    tx.send(val).unwrap();
    tx.send(String::from("asd")).unwrap();
  });

  // let received = rx.recv().unwrap();
  // println!("Got: {}", received);
  // let received = rx.recv().unwrap();
  // println!("Got: {}", received);

  for received in rx {
    println!("Got: {}", received);
  }
}

pub fn try_mutex() {
  let counter = Arc::new(Mutex::new(0));

  let mut handles = vec![];

  for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
      let mut num = counter.lock().unwrap();
      *num += 1;
    });
    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }

  println!("Result: {}", *counter.lock().unwrap());
}
