// use std::env;
// use cargo run in terminal to run
fn main() {
    let strt = get_time() as f64;
    looptest();
    let end = get_time() as f64;
    println!("hi");
    println!("{} elapsed", (end - strt) / 1000_f64);
    maptest();
    rndtest();
}

use std::time::{SystemTime, UNIX_EPOCH};

fn get_time() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let in_ms = since_the_epoch.as_millis();
    // println!("{:?}", in_ms);
    return in_ms;
}

fn looptest() {
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    // 100M in 4 seconds. Uses 1 core (4 cores on my machine);
    // so 25% of "cpu" (since 4 cores on machine).
    // but this is unoptimized with debug info. About 10% faster
    // if optimized.
    let mut a = 1;
    println!("starting...{}", a);
    for _n in 1..100_000_005 {
        a += 1;
    }
    println!("done with 100M loop {}", a);
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

    let mut dict = HashMap::new();
    println!("starting insert of 1M items ");
    for n in 1..1_000_005 {
        dict.insert(n, n + 1);
    }
    println!("ended insert of 1M items {}", dict[&3]); // this is slow because it keeps rehashing

    println!("starting modify of 1M items ");
    for n in 1..1_000_005 {
        dict.insert(n, n - 1);
    }
    println!("ended modify of 1M items {}", dict[&3]); // this is slow because it keeps rehashing

    println!("starting lookup of 1M items ");
    let mut a: i64 = 1; // panic if i32
    for n in 1..1_000_005 {
        a += dict[&n]; // look up numbers.will panic if no entry found
    }
    println!("ended lookup of 1M items {} ", a);

    let mut book_reviews = HashMap::new();

    // Review some books.
    book_reviews.insert("Adventures of Huckleberry Finn", "My favorite book.");
    book_reviews.insert("Grimms' Fairy Tales", "Masterpiece.");
    book_reviews.insert("Pride and Prejudice", "Very enjoyable.");
    book_reviews.insert("The Adventures of Sherlock Holmes", "Eye lyked it alot.");

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
