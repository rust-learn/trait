pub trait Summary {
    fn summarize_auther(&self) -> String;

    // default implementation of `introduce`, which we can keep or override in impl block
    fn summarize(&self) -> String {
        // default implementation can call the methods that don't have a default implementation
        format!("(Read more from {}...)", self.summarize_auther())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_auther(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_auther(&self) -> String {
        format!("@{}", self.username)
    }
}

// a function that takes a param `item` that implements `Summary` trait
// same as:
// pub fn notify<T: Summary>(item: T)
// which is called trait bound
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// For functions that takes more than one param:
// *********************************************************
// pub fn notify(item1: impl Summary, item2: impl Summary)
// *********************************************************
// requires 2 params that implement Summary, but can be of different types

// *********************************************************
//  pub fn notify<T: Summary>(item1: T, item2: T)
// *********************************************************
// requires 2 params of the same type T, where T implements Summary


// Multiple trait bounds with +
// pub fn notify(item: impl Display + Summary)
// or
// pub fn notify<T: Display + Summary>(item: T)


// where clause:
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32
// is equivalent to:
// fn some_function<T, U>(t: T, u: U) -> i32
//    where T: Display + Clone
//          U: Clone + Debug


// Returning Types that implement Traits
fn return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
