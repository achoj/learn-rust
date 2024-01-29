use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String, 
}

impl Summary for NewArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// 将trait作为参数传递
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}


// trait bound
pub fn notify_plus<T: Summary>(item: &T, item2: &T) {
    println!("Breaking news! {}", item.summarize());
}


// 通过where语句来绑定trait
fn some_fuction< T, U>() 
where
    T: Display + Clone,
    U: Clone,
{
    
}