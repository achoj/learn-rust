






fn main() {
    // 整形
    // i8 i16 i32 i64 i128 isize(64 bit) / u*
    // 如果溢出 “two's complement wrapping”.

    // 浮点型
    // f32 f64

    // 字符类型
    // char(4 bytes)

    // 复合类型
    // tuple array
    // tuple:长度固定,
    // 不带有任何值的元组叫做单元，写作(),表示空值或空的返回类型.
    // 如果表达式不返回其他值，会隐式返回单元值.
    let tup: (i32, f64, u8) = (500, 4.5, 12);
    let ( x, y, z) = tup;
    println!("{y}"); 
    let d = tup.2;
    println!("{d}");

    // array
    let months = ["January", "February"];
    // println!("{months}");
    let a: [i32; 5] = [1,2,3,4,5];



}
