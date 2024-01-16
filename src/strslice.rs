fn strslice () {
  let s = String::from("hello world");

  let word_index = str_slice(&s);

  println!("{word_index}");
  // 字符串切片
  let str = String::from("hello world!");
  let hello = &str[..5]; // 可以省略开头与结尾
  let world = &str[6..];
  let hello_world = &str[..]; // 返回整个字符串切片
  println!("{hello} {world} {hello_world}");
}

fn str_slice (str: &String) -> &str {

  for (i, &item) in str.as_bytes().into_iter().enumerate() {
    if item == b' ' {
      return &str[..i];
    }
  }
  return &str[..]
}