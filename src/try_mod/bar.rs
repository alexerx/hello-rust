// 在bar模块内创建一个内部的模块
mod inner_private_mod {
  pub fn inner_private_mod_fn() {
    println!("inner_private_mod_fn");
  }
}

// 每个文件本身就是一个与文件同名的mod
pub fn bar_fn() {
  inner_private_mod::inner_private_mod_fn();
  println!("bar");
}