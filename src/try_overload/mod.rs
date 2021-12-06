use std::ops::Add;

pub fn run() {
  try_overload();
}

// 运算符重载
pub fn try_overload() {
  struct Node {
    name: String,
  }

  impl Add for Node {
    type Output = Node;

    fn add(self, other: Self::Output) -> Self::Output {
      Node {
        name: format!("{}{}", self.name, other.name),
      }
    }
  }

  let n1 = Node { name: "Node1".to_string() };
  let n2 = Node { name: "Node2".to_string() };

  println!("new node's name: {}", (n1 + n2).name);
}