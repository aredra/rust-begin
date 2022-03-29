use trait_example::{Summary, Tweet, NewsArticle};

fn main() {
    let tweet = Tweet {
        username: String::from("ridi_ebooks"),
        content: String::from("of course, as probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Awesome Hacker News!!"),
        location: String::from("Seoul, South Korea"),
        author: String::from("Aredra"),
        content: String::from("Hacker News's apis are awesome!!")
    };

    println!("New article available! {}", article.summarize());
}
