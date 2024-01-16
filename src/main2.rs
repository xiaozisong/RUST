fn main2() {
    println!("Hello, world!");
    // 标准类型
    let val = "  sss  ";
    println!("this is 1st val {val}");
    let val = val.len();
    println!("this is 2nd val {val}");
    let val = val * 2;
    println!("this is 3nd val {val}");
    const MAX_POINTS: u32 = 100_000;
    println!("this is max points {MAX_POINTS}");

    // let val = 256u16;

    // 复合类型
    let tup: (i32, u64, f64) = (12, 32, 2.0);

    let (x, y, z) = tup;

    println!("{x} {y} {z}");
    let new_val = test();
    println!("{new_val}");
}

fn test () -> i32 {
    5
}

