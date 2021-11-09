fn largest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

pub fn try_lifetime_fn() {
  let string1 = "asdf";

  {
    let string2 = "xyz";
    let result = largest(string1, string2);
    println!("The largest string is {}", result);
  }

  let string2 = "xyz";
  let result = largest(string1, string2);
  println!("The largest string is {}", result);
}
