// use std::env;

fn main() {
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    // 100M in 5 seconds. Uses 1 core (4 cores on my machine); so 25% of "cpu" (since 4 cores on machine)
    let mut a = 1;
    println!("{}", a);
    for _n in 1..10000005 {
        a += 1;
    }
    println!("done {}", a);
    maptest();
    rndtest();
}

use std::collections::HashMap;

fn maptest() {
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    for key in map.keys() {
        println!("{}", key);
    }

    // Type inference lets us omit an explicit type signature (which
    // would be `HashMap<String, String>` in this example).
    let mut book_reviews = HashMap::new();

    // Review some books.
    book_reviews.insert(
        "Adventures of Huckleberry Finn".to_string(),
        "My favorite book.".to_string(),
    );
    book_reviews.insert(
        "Grimms' Fairy Tales".to_string(),
        "Masterpiece.".to_string(),
    );
    book_reviews.insert(
        "Pride and Prejudice".to_string(),
        "Very enjoyable.".to_string(),
    );
    book_reviews.insert(
        "The Adventures of Sherlock Holmes".to_string(),
        "Eye lyked it alot.".to_string(),
    );

    // Check for a specific one.
    // When collections store owned values (String), they can still be
    // queried using references (&str).
    if !book_reviews.contains_key("Les Misérables") {
        println!(
            "We've got {} reviews, but Les Misérables ain't one.",
            book_reviews.len()
        );
    }

    // oops, this review has a lot of spelling mistakes, let's delete it.
    book_reviews.remove("The Adventures of Sherlock Holmes");

    // Look up the values associated with some keys.
    let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
    for &book in &to_find {
        match book_reviews.get(book) {
            Some(review) => println!("{}: {}", book, review),
            None => println!("{} is unreviewed.", book),
        }
    }

    // Look up the value for a key (will panic if the key is not found).
    println!("Review for Jane: {}", book_reviews["Pride and Prejudice"]);

    // Iterate over everything.
    for (book, review) in &book_reviews {
        println!("{}: \"{}\"", book, review);
    }

    // type inference lets us omit an explicit type signature (which
    // would be `HashMap<&str, u8>` in this example).
    let mut player_stats = HashMap::new();

    fn random_stat_buff() -> u8 {
        // could actually return some random value here - let's just return
        // some fixed value for now
        42
    }

    // insert a key only if it doesn't already exist
    player_stats.entry("health").or_insert(100);

    // insert a key using a function that provides a new value only if it
    // doesn't already exist
    player_stats
        .entry("defence")
        .or_insert_with(random_stat_buff);

    // update a key, guarding against the key possibly not being set
    let stat = player_stats.entry("attack").or_insert(100);
    *stat += random_stat_buff();
}

fn rndtest() {
    println!("Hello, world!");
    extern crate rand;

    // import commonly used items from the prelude:
    use rand::prelude::*;

    // We can use random() immediately. It can produce values of many common types:
    let x: u8 = random();
    println!("{}", x);

    let y = random::<f64>();
    println!("{}", y);

    if random() {
        // generates a boolean
        println!("Heads!");
    }

    // If we want to be a bit more explicit (and a little more efficient) we can
    // make a handle to the thread-local generator:
    let mut rng = thread_rng();
    if rng.gen() {
        // random bool
        let x: f64 = rng.gen(); // random number in range [0, 1)
        println!("x is: {}", x);
        println!("Random char: {}", rng.gen::<char>());
        println!("Number from 0 to 9: {}", rng.gen_range(0, 10));
    }
}
