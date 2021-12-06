pub fn run() {
  try_sized();
}

pub fn try_sized() {

  fn sized_test<T>(t: T) { }
  fn unsized_test<T: ?Sized>(t: &T) { }

  struct Foo ();

  trait BarTrait {
    fn foo(&self) {}
  }

  let foo = Foo();

  unsized_test(&foo);
  sized_test(foo);
}