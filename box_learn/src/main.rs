use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;


#[derive(Debug)]
enum List {
    // Cons(i32, Box<List>),
    // Cons(i32, Rc<List>),
    // 实现链表，会出现一个节点被多个其它节点共享的情况
    // 要实现相同数据多个所有者，我们使用引用计数Rc来实现
    // 但是多个使用者拥有的只是不可变访问，我们不能修改值
    // 我们使用RefCell来实现，通过对引用计数的RefCell对象
    // 我们可以再次borrow或者mut_borrow来进行修改。
    Cons(Rc<RefCell<i32>>, Rc<List>),
    // 需要进行修改的只是i32数据
    Nil,
}

use std::ops::Deref;

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // println!("Deref called.");
        &self.0
    }
}

/*
* Deref强制转换与可变性交互
* 1. 当T实现了返回U的Deref，可以从&T->&U
* 2. 当T实现了返回U的DerefMut，可以从&mut T -> &mut U
* 3. 当T实现了返回U的Deref，可以从&mut T到 &u
*/

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("MyBox Drop called with value");
    }
}

fn main() {
    let x = 10;

    // 这是对一个由x值的拷贝构造的一个MyBox对象
    // 想要对它实现类似于引用的行为：*取值
    // 需要实现Deref trait
    // *y => *(y.deref()) 通过这样来实现一个引用的效果
    let y = MyBox::new(x);

    // 这是常规引用，类型就是x的引用
    // let y = &x; 可以通过*取值

    // let a: Rc<List> = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));  
    // println!("Count after creating a = {}", Rc::strong_count(&a));
    // let b = Cons(3, Rc::clone(&a));
    // let c = Cons(4, Rc::clone(&a));

    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    let d = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));

    *value.borrow_mut() += 10;
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}