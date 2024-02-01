


// 通常并不需要手动实现 Send 和 Sync trait，
// 因为由 Send 和 Sync 的类型组成的类型，
// 自动就是 Send 和 Sync 的。因为它们是标记 trait，
// 甚至都不需要实现任何方法。
// 它们只是用来加强并发相关的不可变性的。


fn main() {
    println!("Hello, world!");
}
