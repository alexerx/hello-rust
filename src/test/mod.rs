// use std::collections::HashSet;

// 强制在编译期间执行函数 CTFE
// const fn get_len() -> usize {
//     5
// }

// fn closure_math<F: Fn() -> i32>(op: F) -> i32 {
//     op()
// }

// #[derive(Debug)]
// enum Enum {
//     Type1 { x: u32 }
// }

// =================================================================编译时函数执行
    // let arr = [0; get_len()];

    // =================================================================闭包使用外部变量
    // let six = 6;
    // // 函数不能捕获外部变量 can't capture dynamic environment in a fn item
    // // fn add_six(n: i32) -> i32 { n + six }
    // // 应该使用闭包
    // // 感觉闭包和react的useMemo类似。。相当于对一个计算值的封装
    // let closure_add_six = |n| { n + six };
    // // let num = add_six(1);
    // let num = closure_add_six(1);
    // assert_eq!(num, 7);

    // =================================================================闭包作为参数
    // let a = 2;
    // let b = 3;
    // assert_eq!(closure_math(|| a + b), 5)

    // =================================================================条件表达式
    // let a = 2;
    // let b = if a > 1 && a < 10 { a } else { 0 };
    // assert_eq!(b, a);

    // =================================================================match表达式
    // let a = 1;
    // let b = 2;
    // let closure_sum = || a + b;
    // match closure_sum() {
    //     0 => println!("zero"),
    //     3 => println!("right"),
    //     | 1 | 2 => println!("less"),
    //     _ => println!("eeee")
    // }

    // =================================================================while let / if let表达式
    // let a = 1;
    // if let true = a == 1 {
    //     println!("a==1");
    // }
    // let mut v = vec![1,2,3];
    // while let Some(item) = v.pop() {
    //     println!("{}", item);
    // }

    // =================================================================VecDeque
    // let mut buf = VecDeque::new();
    // buf.push_front(1);
    // buf.push_front(2);
    // println!("{:?}", buf.get(0)); // Some(2)
    // println!("{:?}", buf.get(0)); // Some(2)

    // =================================================================泛型结构体
    // #[derive(std::fmt::Debug)]
    // struct Point<T> {
    //     x: T,
    //     y: T,
    // }
    // impl<T> Point<T> {
    //     fn new(x: T, y: T) -> Self {
    //         Point{ x, y }
    //     }
    // }
    // let p1 = Point::new(1, 2);
    // println!("{:?}", p1);

    // =================================================================浮点数陷阱
    // let a = 0.1 + 0.2;
    // assert_eq!(a, 0.3); // left: `0.30000000000000004`, right: `0.3`', IEEE 754的陷阱
    // println!("{:?}", a);
    // if a == 0.3 {
    //     println!("good");
    // } else {
    //     println!("bad");
    // } // bad

    // =================================================================字符串字面量
    // let a = 1;
    // let b = a;
    // println!("{},{}", a, b);
    // let s1 = "s1";
    // let s2 = s1;
    // println!("{},{}", s1, s2);
    // let ss1 = String::from(s1);
    // // let ss2 = ss1;
    // // println!("{},{}", ss1, ss2); // error

    // =================================================================字符串字面量实际上是字符串切片
    // let s: &str = "123";

    // =================================================================结构体
    // #[derive(Debug)]
    // struct Rectangle {
    //     width: u32,
    //     height: u32,
    // }

    // trait NewAndArea {
    //     fn new(width: u32, height: u32) -> Self;
    //     fn area(self: &Self) -> u32;
    // }

    // impl NewAndArea for Rectangle {
    //     fn new (width: u32, height: u32) -> Self {
    //         Rectangle {
    //             width,
    //             height
    //         }
    //     }

    //     fn area(&self) -> u32 {
    //         self.width * self.height
    //     }
    // }
    // fn area(r: &Rectangle) -> u32 {
    //     r.width * r.height
    // }
    // let r1 = Rectangle { width: 66, height: 77 };
    // println!("{}", area(&r1));
    // println!("{:?}", r1);
    // let r2 = Rectangle::new(66, 77);
    // println!("area: {}", r2.area());

    // ===============================================================元组结构体
    // #[derive(Debug)]
    // struct Rectangle (u32, u32);
    // fn area(r: &Rectangle) -> u32 {
    //     r.0 * r.1
    // }
    // let r1 = Rectangle(66, 77);
    // println!("area: {}", area(&r1));
    // println!("{:#?}", r1);

    // =============================================================match Option
    // let a = Some(666); // Option枚举一个int值
    // let s = match a {
    //     Some(666) => 777,
    //     Some(x) => x,
    //     None => 0,
    // };
    // println!("{}", s);

    // =============================================================if let
    // let a = Some(3);
    // if let Some(x) = a {
    //     println!("{}", x);
    // }
    // let s = String::from("str");
    // let ss = Some(&s);
    // println!("{:?}, {:?}", s, ss);

    // =============================================================format!
    // let a = format!("{}--{}++{}", "aa", "bb", "cc");
    // println!("{}", a);

    // =============================================================字符串索引
    // let hello = "Здравствуйте";
    // // 切片
    // let s = &hello[0..1]; // 这里会产生一个panic，因为 З 占用两个字节
    // println!("{}", s);
    // // chars
    // for c in hello.chars() {
    //     println!("{}", c);
    // }

    // =============================================================Result
    // let file = std::fs::File::open("hello.txt");
    // let file = match file {
    //     Ok(f) => f,
    //     Err(e) => match e.kind() {
    //         std::io::ErrorKind::NotFound => match std::fs::File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(ec) => panic!("try to create a file but faild: {:?}", ec),
    //         }
    //         _other_err => panic!("open file other error: {:?}", _other_err),
    //     }
    // };

    // =============================================================Result.unwrap/expect

    // =============================================================generic
    // struct Point<T, U> {
    //     x: T,
    //     y: U,
    // }
    // impl<T, U> Point<T, U> {
    //     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
    //         Point {
    //             x: self.x,
    //             y: other.y,
    //         }
    //     }
    // }
    // let p1 = Point { x: 5, y: 10.4 };
    // let p2 = Point { x: "Hello", y: 'c'};
    // let p3 = p1.mixup(p2);
    // println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // =============================================================generic/trait bound/trait约束/tr
    // trait ATrait {}
    // trait BTrait {}
    // trait XTrait {}
    // trait YTrait {}
    // trait ETrait {
    //     fn say_e(&self) { // 作为trait的方法，必须引用&self，否则只作为trait的关联函数，不能被实例调用
    //         println!("eeee");
    //     }
    // }
    // trait FTrait {
    //     fn say_f(&self) {
    //         println!("ffff");
    //     }
    // }
    // struct Any {}
    // impl ETrait for Any {}
    // impl<T: ETrait> FTrait for T {} // 为所有实现了ETrait的类型实现FTrait，有条件的实现trait
    // let a = Any {};
    // a.say_e();
    // a.say_f();
    // fn add(a: impl ATrait + BTrait, b: impl XTrait + YTrait) -> i32 {
    //     0
    // }
    // fn add1<T: ATrait + BTrait, D: XTrait + YTrait>(a: T, b: D) -> i32 {
    //     0
    // }
    // fn add2<T, D>(a: T, b: D) -> i32
    //     where
    //         T: ATrait + BTrait,
    //         D: XTrait + YTrait,
    // {
    //     0
    // }
    // fn add3<T, D>(a: T, b: D) -> impl ETrait
    //     where
    //         T: ATrait + BTrait,
    //         D: XTrait + YTrait,
    // {
    //     Any {}
    // }

    // =============================================================二分搜索
    // fn search(nums: Vec<i32>, target: i32) -> i32 {
    //     let mut left: i32 = 0;
    //     let mut right: i32 = nums.len() as i32 - 1;
    //     while left <= right {
    //         let mid: i32 = ((right - left) / 2) + left;
    //         let mid_val = nums[mid as usize];

    //         if mid_val == target {
    //             return mid;
    //         }
    //         if mid_val > target {
    //             // target in left
    //             right = mid - 1;
    //         } else {
    //             // target in right
    //             left = mid + 1;
    //         }
    //     }

    //     -1
    // }

    // let nums = vec![5];
    // println!("{}", search(nums, -5));

    // =================================================================set None
    // let a: Option<u32> = None;
    // println!("{}", a.is_none());

    // =================================================================枚举方法
    // enum Json {
    //     Null,
    //     Boolean(bool),
    //     Number(f64),
    //     String(String),
    //     Array(Vec<Json>),
    // }

    // // 甚至可以为枚举添加方法
    // impl Json {
    //     fn fuck(&self) {
    //         println!("fuck")
    //     }
    // }

    // let str = Json::Number(1.0);
    // str.fuck();
    
    
    // let a = Enum::Type1 { x: 1 };
    // println!("{:?}", a);