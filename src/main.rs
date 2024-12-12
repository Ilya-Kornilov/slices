fn main() {
    let tweet = String::from("This is my tweet and it's too long!");
    let trimmed_tweet = &tweet[..20];

    println!(" > tweet: {tweet}");
    println!(" > trimmed tweet: {trimmed_tweet}.");

    let trimmed_tweet_1 = trim_tweet(&tweet);
    println!(" > trimmed tweet 1: {trimmed_tweet_1}.");
    
    let tweet_2 = "This is my tweet and it's too long!";
    let trimmed_tweet_2 = trim_tweet_2(&tweet_2);
    println!(" > trim for tweet 2: {trimmed_tweet_2}...");

    // passing &String to fn ...2 (as a &str)
    let trimmed_tweet_3 = trim_tweet_2(&tweet);
    println!(" > trim for tweet with the same fn: {trimmed_tweet_3}...");
}

fn trim_tweet(tweet: &String) -> &str {
    &tweet[..16]
}

fn trim_tweet_2(tweet: &str) -> &str {
    &tweet[..11]
}
