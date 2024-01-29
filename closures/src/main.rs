
use std::thread;

// the shirt color
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

// fn trait
// FnOnce:适用于能被调用一次的闭包
// FnMut: 适用于不会将捕获的值移出闭包体的闭包，但是可能会修改
// 捕获的值，适用于调用多次的值。
// Fn: 适用于适用于既不将被捕获的值移出闭包体也不修改被捕获的值的闭包，
// 当然也包括不从环境中捕获值的闭包


struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveawsy(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Blue => num_blue += 1,
                ShirtColor::Red => num_red += 1,
            }
        }
        if num_blue > num_red {
            ShirtColor::Blue
        } else {
            ShirtColor::Red
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue]
    };

    let user1 = Some(ShirtColor::Red);


    let list = vec![1, 2, 3];
    println!("Before defining closure {:?}",list);

    let only_borrows = || println!("From closure {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    thread::spawn(move || println!("From main thread: {:?}", list)).join().unwrap();
}


    // 这里错误的原因是：一次将value所有权通过operation移出闭包的时候，
    // 再次调用value已经不在了。
    // let mut sort_operations = vec![];
    // let value = String::from("by key called");
    // list.sort_by_key(|r| {
    //     sort_operations.push(value);
    //     r.width
    // });

    // 这里只是用了可变引用，其他均未变。
    // let mut num_sort_operations = 0;
    // list.sort_by_key(|r| {
    //     num_sort_operations += 1;
    //     r.width
    // });