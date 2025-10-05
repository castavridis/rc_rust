/**
 * https://projecteuler.net/problem=3
 * 
 * The prime factors of 13195 are 5, 7, 13 and 29.
 * What is the largest prime factor of the number 600851475143?
 */

 use std::io;

 // A prime number is a number that can only be divided by 1 or itself
fn is_prime(value: i64, primes: Vec<i64>) -> bool {
  for prime in primes {
    if value % prime == 0 {
      return false; 
    }
  }
  return true;
}

fn main () {
  let mut primes: Vec<i64> = Vec::new();
  // Start at 2 because we know numbers > 1 are divisible by 1
  let mut value: i64 = 2;
  let mut ceiling: String = String::new();
  io::stdin()
    .read_line(&mut ceiling) // Why do I have to prepend &mut? To let read_line know it's borrowing ceiling and can mutate it?
    .expect("Failed to read line.");
  primes.push(2);
  while value <= ceiling.trim().parse::<i64>().unwrap() {
    // let is = is_prime(value, primes.clone());
    // println!("{}, {}", value, is);
    if is_prime(value, primes.clone()) {
      println!("{}", value);
      primes.push(value);
    }
    value += 1;
  }
  println!("Largest prime: {:#?}", primes.last());
}