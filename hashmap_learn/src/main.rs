
use std::collections::HashMap;


fn main() {
    let mut score = HashMap::new();

    score.insert(String::from("Blue"), 10);
    score.insert(String::from("Yellow"), 34);

    for i  in score.keys() {
        println!("{i}");
    }
 
}
