

fn another_funtion() {
    println!("Hello from another_function.");
}

fn print_labeled_(value: u32, unit_label: char) {
   println!("The measurement is : {value}{unit_label}"); 
}
// Tes
// rust中函数不显示return,通过表达式返回值来返回，因此不加分号
// 因为加了分号就是语句而不是表达式了，如果没有表达式返回语句
// 默认返回().
fn return_func() -> i32 {
   12
}

fn main() {
    println!("Hello, world!");
    another_funtion();
    print_labeled_(12, 'h');

    let y = return_func();
    println!("{y}");
    println!("");
}
