/**
 * https://projecteuler.net/problem=3
 * 
 * The prime factors of 13195 are 5, 7, 13 and 29.
 * What is the largest prime factor of the number 600851475143?
 */


 use std::io;

 // It is not considered good form to keep globals around, apparently the reason
 // is because it's hard to test with globals?
 
 // Convert this recursive function to a while loop
 fn find_prime_factors (primes: &Vec<u64>, factors: &mut Vec<u64>, ceiling: u64) {
  let mut i = 0;
  while i < primes.len() {
    let prime: &u64 = primes.get(i).unwrap();
    if ceiling % *prime == 0 {
      println!("Prime factor found: {}.", *prime);
      factors.push(*prime);
    }
    i += 1;
  }
 }
 
  // A prime number is a number that can only be divided by 1 or itself
 fn is_prime(value: u64, primes: Vec<u64>) -> bool {
   for prime in primes {
     if value % prime == 0 {
       return false; 
     }
   }
   return true;
 }
 
 fn main () {
   // Get value to evaulate from input
   println!("\nThis program attempts to find the largest prime factor of a given number.");
   println!("\nWhat number would you like to evaluate?");
   let mut input = String::new();
   io::stdin()
     .read_line(&mut input)
     .expect("Failed to read line.");
   let value: f64 = input.trim().parse::<f64>().unwrap();
   
  // Get the square root of the value, numbers below the square root are potential factors
   
  // Saurabh Jain (https://math.stackexchange.com/users/167164/saurabh-jain), 
  // Is a prime factor of a number always less than its square root?, 
  // URL (version: 2020-09-10): https://math.stackexchange.com/q/883184 
   let sqrt: f64 = value.sqrt();
   let sqrt_floor: u64 = sqrt.floor() as u64;
   println!("\nFinding primes <= the square root of {}, floored ({})...", value, sqrt_floor);
 
   // Get all primes less than the square root
   let mut test: u64 = 2;
   let mut primes: Vec<u64> = Vec::new();
   while test <= sqrt_floor {
     if is_prime(test, primes.clone()) {
       primes.push(test);
     }
     test += 1;
   }

   if primes.len() == 0 {
    println!("\nCould not find prime for {}.", value);
    return
   }
   
   // For each prime check if it is a factor
   let mut factors: Vec<u64> = Vec::new();
   println!("\nFinding prime factors for {}...\n", value);
   find_prime_factors(&primes, &mut factors, value as u64);
   if factors.len() > 0 {
     println!("\nThe largest prime factor for {} is {}.", value, factors.last().unwrap());
   } else {
     println!("\nCould not find prime factors for {}.", value);
   }
   println!();
 }