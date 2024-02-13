struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

struct NewsArticle {
    author: String,
    content: String,
    location: String,
}

pub trait Summary {
    fn summarize(&self) -> String {
        // Default implementation
        String::from("(Read more...)")
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.content, self.author, self.location)
    }
}


fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    let article = NewsArticle {
        author: String::from("John Doe"),
        content: String::from("This is a news article"),
        location: String::from("New York"),
    };
    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new article: {}", article.summarize());
}

