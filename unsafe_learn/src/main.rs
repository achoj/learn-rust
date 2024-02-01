

// 解引用裸指针
// 调用和实现不安全方法
// 访问或修改可变静态变量

unsafe fn print_unsafe_pointer(i: *mut i32) {
    println!("Unsafe varlue: {}", *i);
}

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        print_unsafe_pointer(r2);
    }
}
