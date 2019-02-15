fn main() {
    println!("Hello, world!");
    println!("{}", 1 + 3);
//    loop {
//        println!("Hello, world!");
//    }

    // ループ
    // https://doc.rust-jp.rs/the-rust-programming-language-ja/1.6/book/loops.html

    let mut i = 0;
    while i < 5 {
        i += 1;
        println!("{}", i);
    }

    // output : 0 1 2 3 4
    for i in 0..5 {
        println!("{}", i);
    }

    // iにindexが入る
    for (i, j) in (5..10).enumerate() {
        println!("i = {} and j = {}", i, j);
    }
}