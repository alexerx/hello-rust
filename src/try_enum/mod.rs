#[allow(dead_code)]
enum Shape {
  Square { width: u32 },
  Rectangle { width: u32, height: u32 },
  Circle { radio: u32 },
}

pub fn try_enum_destruction() {
  // 枚举关联值的解构
  if let Shape::Square{width} = (Shape::Square{width: 66}) {
		println!("width: {}", width);
	}
}