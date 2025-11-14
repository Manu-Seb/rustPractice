use std::io;

pub trait Summary {
    fn summarize(&self) -> String {
        let mut st = String::new();
        st.push_str("Read More");
        return st;
    }
}

struct NewsArticle {
    pub headline: String,
    pub content: String,
    pub footer: String,
    pub author: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        return format!(
            "{}{}{}{}",
            &self.headline, &self.content, &self.footer, &self.author
        );
    }
}

impl NewsArticle {
    fn new(headline: &str, content: &str, footer: &str, author: &str) -> Self {
        Self {
            headline: headline.to_string(),
            content: content.to_string(),
            footer: footer.to_string(),
            author: author.to_string(),
        }
    }
}

fn notify_user(item: &impl Summary) {
    println!(
        "The user has been notified of the article : {}",
        item.summarize()
    );
}

pub fn test_news() {
    let mut headline = String::new();
    let mut content = String::new();
    let mut footer = String::new();
    let mut author = String::new();

    io::stdin()
        .read_line(&mut headline)
        .expect("Could not read line");
    io::stdin()
        .read_line(&mut content)
        .expect("Could not read line");
    io::stdin()
        .read_line(&mut footer)
        .expect("Could not read line");
    io::stdin()
        .read_line(&mut author)
        .expect("Could not read line");

    let news: NewsArticle = NewsArticle::new(&headline, &content, &footer, &author);

    let summary = news.summarize();

    println!("The summary is \n{}", summary);
    notify_user(&news);
}
