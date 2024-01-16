// 使用接口
#[derive(Debug)]
// 利用struct
struct Rectangle {
  width: u32,
  height: u32,
}
fn main () {
  // 使用一个函数，输出长方形的面积
  let Rect = Rectangle {
    width: 30,
    height: 50,
  };

  println!("{}", area(&Rect));
  // 如何打印struct 直接打印会报错，需要借用一个接口
  // println!("{}", Rectangle);
  // 调用接口后，再打印, 使用#可以使打印的值更加清晰
  println!("{:#?}", Rect);
}

fn area (rect: &Rectangle) -> u32 {
  rect.width * rect.height
}