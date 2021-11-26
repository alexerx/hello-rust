struct Cache<T>
where
  T: Fn(u32) -> u32,
{
  calculation: T,
  value: Option<u32>,
}

impl<T> Cache<T> where T: Fn(u32) -> u32 {
  fn new(calculation: T) -> Cache<T> {
    Cache {
      calculation,
      value: None,
    }
  }

  fn value(&mut self, arg: u32) -> u32 {
    match self.value {
      Some(v) => v,
      None => {
        let v = (self.calculation)(arg);
        self.value = Some(v);
        v
      }
    }
  }
}

struct Point {
  x: u32,
  y: u32,
}

pub fn try_closure_fn() {
  let mut cache = Cache::new(|a| a+1);
  println!("{}", cache.value(1));
  println!("{}", cache.value(2));

  let p = Point { x: 1, y: 2 };
  let get_x = |p: &Point| p.x;
  println!("x: {}", get_x(&p));
}
