use std::fmt::Display;

pub fn run() {
  try_newtype();
}

pub fn try_newtype() {
  // impl Display for String { } // error，孤儿规则

  struct WrappedString(String);

  impl Display for WrappedString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "STRING: {}", self.0)
    }
  }

  let ws = WrappedString(String::from("rust"));

  println!("{}", ws); // STRING: rust
}