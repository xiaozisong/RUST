// 调用trait
#[derive(Debug)]
// 了解struct的方法
struct Rectangle {
  width: u32,
  height: u32,
}
// 利用struct
impl Rectangle {
  fn area (&self) -> u32 {
    self.width * self.height
  }
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width * self.height > other.width * other.height
  }
  // 创建关联函数，是函数不是方法，例如String::from()
  fn square (size: u32) -> Rectangle {
    Rectangle {
      width: size,
      height: size,
    }
  }
}
fn main () {
  // 使用一个函数，输出长方形的面积
  let Rect1 = Rectangle {
    width: 30,
    height: 50,
  };

  let Rect2 = Rectangle {
    width: 20,
    height: 40,
  };

  let Rect3 = Rectangle {
    width: 40,
    height: 60,
  };

  let square = Rectangle::square(20);

  println!("{}", Rect1.area());

  println!("{}", Rect1.can_hold(&Rect2));

  println!("{}", Rect1.can_hold(&Rect3));

  println!("{:#?}", square);
}

