struct Counter {
  count: u32,
}

impl Counter {
  fn new() -> Counter {
    Counter { count: 0 }
  }
}

impl Iterator for Counter {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    self.count += 1;
    if self.count < 6 {
      Some(self.count)
    } else {
      None
    }
  }
}

pub fn try_iter_fn() {
  let counter = Counter::new();

  for item in counter {
    println!("{}", item);
  }

  let sum: u32 = Counter::new().take(5).sum();
  println!("Sum: {}", sum);
}