/**
 * https://projecteuler.net/problem=3
 * 
 * The prime factors of 13195 are 5, 7, 13 and 29.
 * What is the largest prime factor of the number 600851475143?
 */


use std::io;

// It is not considered good form to keep globals around, apparently this is for testing?

fn factorialization_loop (primes: &Vec<i32>, factors: &mut Vec<i32>, ceiling: f64, index: usize) {
  match primes.get(index) {
    Some(prime) => {
      // let mut _ceiling = ceiling;
      // let mut is_factor = false;
      // while _ceiling % *prime as f64 == 0.0 && _ceiling != 0.0 {
        // if !is_factor { 
        if ceiling % *prime as f64 == 0.0 {
          println!("Prime factor found: {}.", prime);
          factors.push(*prime);
          // is_factor = true;
        }
        // _ceiling = _ceiling / *prime as f64;
      // }
      factorialization_loop(primes, factors, ceiling, index + 1);
    }
    None => return
  }
}

 // A prime number is a number that can only be divided by 1 or itself
fn is_prime(value: i32, primes: Vec<i32>) -> bool {
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
  
  /**
   * Get the square root of the value, numbers below the square root are potential factors
   * 
   * Saurabh Jain (https://math.stackexchange.com/users/167164/saurabh-jain), 
   * Is a prime factor of a number always less than its square root?, 
   * URL (version: 2020-09-10): https://math.stackexchange.com/q/883184
   */ 
  let sqrt: f64 = value.sqrt();
  let sqrt_floor: i32 = sqrt.floor() as i32;
  println!("\nFinding primes <= the square root of {}, floored ({})...", value, sqrt_floor);

  // Get all primes less than the square root
  let mut test: i32 = 2;
  let mut primes: Vec<i32> = Vec::new();
  while test <= sqrt_floor {
    if is_prime(test, primes.clone()) {
      primes.push(test);
    }
    test += 1;
  }
  
  // For each prime check if it is a factor
  let mut factors: Vec<i32> = Vec::new();
  println!("\nFinding prime factors for {}...\n", value);
  factorialization_loop(&primes, &mut factors, value, 0);

  println!("\nThe largest prime factor for {} is {}.", value, factors.last().unwrap());
  println!();
}