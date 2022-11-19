use std::io;
use std::time::{Instant, SystemTime};

fn main() {

    let mut primeVector :Vec<u64> =  Vec::new();
    primeVector = vec![2];
    let mut number = 3;
    let mut primeBool = true;
    let mut counter = 0;
    let now = Instant::now();

    while counter < 25000{
        primeBool = true;
        for prime in &primeVector{
            if number % prime == 0 {
                primeBool = false;
                break;
            }
        }
        if primeBool{
            primeVector.push(number);
            counter+=1;
        }
        number += 2;
    }
    let temp:f64 = now.elapsed().as_nanos() as f64;
    println!("{}",  temp/1000000000.0);
}
