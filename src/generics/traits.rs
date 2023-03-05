pub struct NewArticle {
    pub _headline: String,
    pub _location: String,
    pub _author: String,
    pub _content: String,
}

pub struct Tweet {
    pub _username: String,
    pub _content: String,
    pub _reply: bool,
    pub _retweet: bool,
}

pub trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for NewArticle {
    fn summarize(&self) -> String {
        return format!(
            "{}, by {}, ({})",
            self._headline, self._author, self._location
        );
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        return format!("{}: {}", self._username, self._content);
    }
}

pub fn _exec_it() {
    let my_tweet = Tweet {
        _username: "codeYan".to_string(),
        _content: "This is my first tweet :))".to_string(),
        _reply: false,
        _retweet: false,
    };

    let my_article = NewArticle {
        _headline: "Y'all are fuckers!".to_string(),
        _location: "Somewhere".to_string(),
        _author: "Yanzada".to_string(),
        _content: "Do you really need to know?".to_string(),
    };

    let article_value = my_article.summarize();
    let tweet_value = my_tweet.summarize();

    println!("{article_value}");
    println!("{tweet_value}");
}

pub fn _test_it(it: &impl Summary) -> String {
    return it.summarize()
}
