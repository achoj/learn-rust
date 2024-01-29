use trait_learn::{notify, notify_plus, Summary, Tweet};


fn main() {
    let tweet = Tweet {
        username: String::from("TestUser"),
        content: String::from(
            "I am learning rust."
        ),
        reply:false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    notify(&tweet);
    notify_plus(&tweet, &tweet);
}
