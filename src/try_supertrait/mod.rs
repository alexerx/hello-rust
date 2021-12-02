use std::fmt::Display;

pub fn run() {
  try_supertrait();
}

pub fn try_supertrait() {
  trait OutlinePrint: Display {
    fn outline_print(&self) {
      let output = self.to_string();
      let len = output.len();
      println!("{}", "*".repeat(len + 4));
      println!("*{}*", " ".repeat(len + 2));
      println!("* {} *", output);
      println!("*{}*", " ".repeat(len + 2));
      println!("{}", "*".repeat(len + 4));
    }
  }

  impl OutlinePrint for String {}

  let s = String::from("Rust");

  s.outline_print();
}