struct Tweet {
	pub username: String,
	pub content: String,
}

pub trait Summary {
	fn summarize(&self) -> String;

  // 提供默认实现
	fn greet(&self) -> String {
		format!("hello...")
	}
}

impl Summary for Tweet {
	fn summarize(&self) -> String {
		format!("{}: {}", self.username, self.content)
	}
}

pub fn try_trait_fn() {
  let tweet = Tweet {
    username: String::from("Elon Mask"),
    content: String::from("To the moon!"),
  };
  println!("{}", tweet.greet());
  println!("1 new tweet: {}", tweet.summarize());
}