mod bar; // 前往加载bar.rs文件中的内容

// 将路径导入当前作用域
use bar::bar_fn;

pub fn foo_fn() {
  println!("call bar first:");
  
  // 使用绝对路径执行函数
  crate::try_mod::bar::bar_fn();
  // 使用相对路径执行函数
  bar::bar_fn();
  // 使用use导入的路径执行函数
  bar_fn();

  println!("then call foo:");
  println!("foo");
}