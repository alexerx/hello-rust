use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

pub fn try_box() {
  let b = Box::new(5);
  let b_in_stack = 5;
  // let bb = Box::new(b); // moved
  let bb = Box::new(b_in_stack);
  println!("b = {}, bb = {}", b, bb);
}

fn test_deref_coercion(s: &str) {
  println!("{}", s);
}

pub fn try_my_box() {
  struct MyBox<T>(T);

  impl<T> MyBox<T> {
    fn new(v: T) -> MyBox<T> {
      MyBox(v)
    }
  }

  impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
      println!("deref in MyBox");
      &self.0
    }
  }

  let a = MyBox::new(5);
  assert_eq!(5, *a);

  let b = MyBox::new(String::from("hello"));
  test_deref_coercion(&b); // &b自动发生了连续的解引用转换： &MyBox<String> -> &String -> &str，（因为标准库为String提供的Deref trait实现返回&str）
}

pub fn try_drop_trait() {
  struct CustomSmartPointer {
    data: String,
  }

  impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
      println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
  }

  let c = CustomSmartPointer {
    data: String::from("my stuff"),
  };
  println!("CustomSmartPointer created.");
  drop(c); // 主动提前清理变量c。否则会在程序结束时自动清理
  println!("CustomSmartPointer dropped before the end of main.");
}

pub fn try_rc() {
  // let a = String::from("hello");
  // let b = a;
  // let c = a; // error: use of moved value: `a`

  let a = Rc::new(String::from("hello"));
  let _a = Rc::new(String::from("hello"));
  let _aa = Rc::new(String::from("hello"));
  let _aaa = Rc::new(String::from("hello"));
  println!("counter for a: {}", Rc::strong_count(&a));
  let _b = Rc::clone(&a); // clone会增加引用计数
  println!("counter for a: {}", Rc::strong_count(&a));

  {
    let _c = Rc::clone(&a);
    println!("counter for a: {}", Rc::strong_count(&a));
  }

  println!("counter for a: {}", Rc::strong_count(&a));
}

pub fn try_ref_cell() {
  struct Node {
    name: String,
    contents: RefCell<Vec<String>>,
  }

  impl Node {
    fn new(name: String) -> Node {
      Node {
        name,
        contents: RefCell::new(vec![]),
      }
    }

    fn add(&self, content: &str) {
      self.contents.borrow_mut().push(String::from(content));
    }
  }

  let node = Node::new(String::from("root"));
  node.add("shit");

  println!("{:?}", node.contents.borrow());
}

pub fn try_make_a_tree() {
  #[derive(Debug)]
  struct Node {
    name: String,
    parent: RefCell<Weak<Node>>, // 逻辑上，父节点应当拥有子节点的所有权，但是子节点不应当拥有父节点的所有权。并且这样可以避免循环引用
    children: RefCell<Vec<Rc<Node>>>,
  }

  let root = Rc::new(Node {
    name: String::from("root"),
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![]),
  });

  println!(
    "leaf strong = {}, weak = {}",
    Rc::strong_count(&root),
    Rc::weak_count(&root),
  );

  let leaf = Rc::new(Node {
    name: String::from("leaf"),
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![]),
  });

  println!(
    "leaf strong = {}, weak = {}",
    Rc::strong_count(&root),
    Rc::weak_count(&root),
  );

  root.children.borrow_mut().push(Rc::clone(&leaf));
  *leaf.parent.borrow_mut() = Rc::downgrade(&root);

  println!(
    "leaf strong = {}, weak = {}",
    Rc::strong_count(&root),
    Rc::weak_count(&root),
  );

  println!("{:?}", root);
}

pub fn run() {
  // try_box();
  // try_my_box();
  // try_drop_trait();
  // try_rc();
  // try_ref_cell();
  try_make_a_tree();
}
