fn main() {
    let mut s = String::from("Hello ");

    s.push_str("world.");

    println!("{s}");    
    
    let s1 = String::from("Hello");
    
    //let s2 = s1;
    // println!("{s1}"); error
    // rust不会存在潜在的深拷贝.

    let s3 = s1.clone();
    println!("{s3}, ,{s1}");
    
    // 可以Copy的类型，也即无深浅拷贝的类型
    // 所有整数类型
    // 布尔类型
    // 所有浮点类型
    // 元组（当且仅当其包含的所有类型都实现了Copy的时候。）

}
