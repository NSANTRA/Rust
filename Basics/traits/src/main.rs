// Trait (Interface) for custom data type
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

// Trait (Interface) for f64 data type
pub trait ModifyNumber {
    fn modify_number(&self) -> usize;
}

// Custom data type
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Summary for NewsArticle {}

impl ModifyNumber for f64 {
    fn modify_number(&self) -> usize {
        *self as usize
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Headline"),
        location: String::from("Location"),
        author: String::from("Author"),
        content: String::from("Content")
    };

    println!("News Article: {}", article.summarize());

    let tweet = Tweet {
        username: String::from("Username"),
        content: String::from("Content"),
        reply: true,
        retweet: true
    };

    println!("Tweet: {}", tweet.summarize());

    let num: f64 = 10.7;

    println!("Removed fraction part: {}", num.modify_number());
}