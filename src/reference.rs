fn references () {
  let s1 = gives_ownership();
  let s2 = String::from(" world "); 
  let s3 = takes_and_gives(s2); // s2所有权已经交给s3,所以s2已经被drop掉了,再次使用s2会导致编译报错
  println!("s1: {s1}, s3: {s3}");
}

fn gives_ownership () -> String {
  let some_string = String::from(" hello ");
  some_string
}

fn takes_and_gives (some_string: String) -> String {
  some_string
}