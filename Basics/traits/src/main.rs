use std::fmt::{Debug};

// Trait (Interface) for custom datatype
pub trait Summary {
    fn mention_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more... {})", self.mention_author())
    }
}

// Trait (Interface) for f64 data type
pub trait ModifyNumber {
    fn modify_number(&self) -> usize;
}

// Custom datatype
#[derive(Debug)]
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
    fn mention_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.mention_author(), self.content)
    }
}

impl Summary for NewsArticle {
    fn mention_author(&self) -> String {
        format!("@{}", self.author)
    }
}

impl ModifyNumber for f64 {
    fn modify_number(&self) -> usize {
        *self as usize
    }
}

// fn notify(item: &impl Summary) {
//     // Alternate way for trait bound is:
//     // fn notify<T: Summary>(item: &T) {}
//     println!("Breaking news! {}", item.mention_author());
// }

// Second alternative for trait bound syntax
fn notify<T>(item: &T)
where
    T: Summary {
    println!("{}", item.summarize());
}

fn return_traits(item: NewsArticle) -> impl Summary + Debug{
    item
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Headline"),
        location: String::from("Location"),
        author: String::from("Author"),
        content: String::from("Content")
    };

    println!("News Article: {}", article.summarize());

    println!("News Article 2: {}", return_traits(article).summarize());
    // println!("News Article 2: {:#?}", return_traits(article));
    // return_traits(article).summarize();

    let tweet = Tweet {
        username: String::from("Username"),
        content: String::from("Content"),
        reply: true,
        retweet: true
    };

    println!("Tweet: {}", tweet.summarize());

    notify(&tweet);

    let num: f64 = 10.7;

    println!("Removed fraction part: {}", num.modify_number());
}
