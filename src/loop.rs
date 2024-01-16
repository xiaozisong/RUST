fn xunhuan () {
  // loop {
  //   println!(" again! ");
  // };
  let mut counter = 0;
  let number = loop {
      counter += 1;
      if counter == 10 {
        break counter * 2;
      }
  };
  println!("number is {number}");
}