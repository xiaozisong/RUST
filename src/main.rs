// struct
fn main () {
  struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
  }
  // 如果想更新struct里面的属性值，可以使用mut关键字进行标识，但是rust不允许只修改某一个字段
  // 用mut标识后所有成员字段都可以被修改
  let mut user = User {
    username: String::from("shawn"),
    email: String::from("123@gmail.com"),
    active: true,
    sign_in_count: 556,
  };
  // 和其他语言中访问属性的形式是一样的，都是通过  “.” 
  user.email = String::from("345@gmail.com");

  // struct的更新方法  如果想使用前一个struct的值，不需要这么繁琐，写法和作用与js差不多 通过..的方式进行赋值
  let user2 = User {
    username: String::from("zis"),
    email: String::from("567@gmail.com"),
    active: user.active,
    sign_in_count: user.sign_in_count,
  };

  let user3 = User {
    username: String::from("lisi"),
    email: String::from("798@gmail.com"),
    ..user2
  };

  // tuple struct 元祖struct 和元祖的使用方法一样，可以解构可以通过索引访问
  struct Color (i32, i32, i32);
  struct Point (i32, i32, i32);
  let color = Color(1,2,3);
  let origin = Point(0, 0, 0);
  let color1 = color.0;
  let (a, b, c) = (1,2,3);

}