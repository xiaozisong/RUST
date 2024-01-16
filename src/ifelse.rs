fn condition () {
  let num = 6;
  if num > 6 {
    println!(" num is bigger than 6 ");
  } else {
    println!(" num is smaller than 6 ");
  }

  let condition = true;
  let number = if condition { 3 } else { 4 };
  println!(" number is {number} ");
}