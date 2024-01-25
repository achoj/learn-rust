

// 引用默认不可变，可以存在多个
// 可变引用只能存在一个，且不能与不可变引用同时使用.


/**
 * 这里有一个编程小习题：编写一个函数，
 * 该函数接收一个用空格分隔单词的字符串，并返回在
 * 该字符串中找到的第一个单词。
 * 如果函数在该字符串中并未找到空格，则整个字符串就是一个
 * 单词，所以应该返回整个字符串。
 */


fn without_slice(s: &String) ->usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate()  {
       if item == b' '  {
           return i;
       } 
    }

    s.len()
}

fn with_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
       if item == b' ' {
        return &s[0..1];
       } 
    }
    &s[..]
}

fn main() {
    let mut s = String::from("Hello world");

    // 这个函数的返回值的状态跟s的状态实际上是脱离的
    // 也就是s即使改变了，凡是word函数原先的值。
    let word = without_slice(&s);
    println!("The first ' ' is {word}");
    s.clear();


    let s1 = String::from("Hello world");
    let hello = &s1[0..5];
    let world = &s1[6..11];
}
