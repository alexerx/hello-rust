use std::collections::BTreeMap;

#[allow(dead_code)]
#[derive(Debug)]
enum JsonType {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonType>),
    Object(BTreeMap<String, JsonType>),
}

// 用枚举类型在动态数组中存储不同类型的值
pub fn try_enum_in_vec() {
  let mut json_token = vec![
    JsonType::String("hello".to_string()),
    JsonType::Boolean(true),
  ];

  // let first_token = json_token.get_mut(0);
  // match first_token {
  //   Some(JsonType::String(s)) => println!("first token in vec is: {:?}", s),
  //   _ => println!("first token in vec is not found"),
  // }

  // if let 语法糖
  // if let 模式 = 表达式 { 代码块 }
  // = 这里理解为模式与表达式的分隔符，并没有相等的意思
  // 如果模式与表达式相匹配，则执行代码块
  if let Some(JsonType::String(first_token)) = json_token.get_mut(0) {
    println!("first token in vec is: {:?}", first_token);
  } else {
    println!("first element in vec is not found");
  }
}