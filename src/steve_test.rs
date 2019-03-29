// use std::env;

fn main() {
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    let mut a=1;
    println!("starting");
    for _n in 1..10000000003 {
        a+=1;   
    }
    println!("done {}",a)
}