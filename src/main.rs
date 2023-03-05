mod generics;
use generics::traits::{Tweet, _exec_it, self};

fn main() {
    let my_tweet = Tweet {
        _username: "codeYan".to_string(),
        _content: "This is my first tweet :))".to_string(),
        _reply: false,
        _retweet: false,
    };

    let value = traits::_test_it(&my_tweet);

    println!("{}", value);

    _exec_it();
}
