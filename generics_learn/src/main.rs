


fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}


impl<T> Point<T> {
    fn x(&self) -> &T {
       &self.x 
    }
}
// Error!
// impl<T> Point<T> {
//     fn distance_calu(&self) -> &T {
//         println!("call generic function.");
//         &self.x()
//     }
// }

impl Point<f32> {
    fn distance_calu(&self) -> f32 {
        println!("call the f32 only function.");
        self.x().sqrt()
    }
}


fn main() {
    let numlist = vec![12,23,34,566,767,324,1231,3];
    let result = largest(&numlist);
    println!("{result}");

    let char_list = vec!['y', 't', 'm', 'b'];
    let result  = largest(&char_list);
    println!("{result}");

    let p = Point {x: 10, y: 12};
    println!("p.x = {}", p.x());
}
