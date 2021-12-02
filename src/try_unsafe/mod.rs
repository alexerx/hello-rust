use std::slice::from_raw_parts_mut;

pub fn run() {
  try_raw_pointer();
  try_unsafe_fn();
  try_static_variable();
  try_unsafe_trait();
}

pub fn try_raw_pointer() {
  let mut num = 5;
  let r1 = &num as *const i32;
  let r2 = &num as *const i32;
  let r3 = &mut num as *mut i32;

  unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
    println!("r3 is: {}", *r3);
  }
}

pub fn try_unsafe_fn() {
  fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
  
    assert!(mid <= len);
  
    unsafe {
      (
        from_raw_parts_mut(ptr, mid),
        from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
      )
    }
  }

  println!("second slice length: {}", split_at_mut(&mut[1,2,3,4,5], 3).1.len());
}

pub fn try_static_variable() {
  static mut HELLO: &str = "hello";

  unsafe {
    println!("{}", HELLO);
  }

  unsafe {
    HELLO = "world";
  }
}

pub fn try_unsafe_trait() {
  unsafe trait Foo {
    fn foo(&self);
  }

  unsafe impl Foo for i32 {
    fn foo(&self) {
      println!("i32");
    }
  }
}




