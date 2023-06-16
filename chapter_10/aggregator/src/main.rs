use aggregator::{NewsArticle, Summary, Tweet};

fn print_summary(summarizable: &impl Summary) {
    println!("{}", summarizable.summarize());
}

fn main() {
    let tweet = Tweet {
        username: String::from("nazmul"),
        content: String::from("Hello rust!"),
        reply: false,
        retweet: false,
    };
    print_summary(&tweet);

    let news = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    print_summary(&news);
}
