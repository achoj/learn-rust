
// 主要目标：避免悬垂引用。
// 生命周期标识并不改变任何生命周期
// 只是通过标识向编译器说明变量的操作是安全的。

// error!
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     }else {
//         y
//     }
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }else {
        y
    }
}

fn main() {
    let x  = String::from("Hello");
    let y = String::from("worlddffjjjjj");

    println!("The longest line is {}", longest(&x, &y));
}
