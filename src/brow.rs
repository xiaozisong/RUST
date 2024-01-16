fn brow () {
  let mut s = String::from(" hello ");
  // 引用：两个条件只能满足其一。1. 存在多个不可变的引用。2.存在可变的引用。
  let s1 = &s;
  let s2 = &s;
  // 但是可以创建新的作用域去解决这个问题
  {
    let s3 = &mut s;
  }
  println!("{s1}, {s2}");

  let r = dangle();
  println!("{r}");
}

// 悬空引用 函数内s变量在20行之后已经脱离作用域并被drop了，但是函数会返回一个空引用，这回导致编译报错
fn dangle () -> &String {
  let s = String::from("world");
  &s
}
