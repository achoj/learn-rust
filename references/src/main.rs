



fn calculate_length(s: &String) -> usize {
   s.len() 
}




fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length is {len}");

    // 引用默认不可变，但能存在多个，可变引用只能存在一个，且两者不能同时存在。
    // 可以减小数据竞争


    // 如果不使用引用的话
    // s1传入函数后，所有权转移将不能继续使用
    // 除非使用tuple将所有权跟函数返回值一起返回
}
