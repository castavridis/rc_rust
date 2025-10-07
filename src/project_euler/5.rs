/**
 * https://projecteuler.net/problem=5
 * 
 * 2520 is the smallest number that can be divided by each of the numbers from
 * 1 to 10 without any remainder.
 * 
 * What is the smallest positive number that is evenly divisible by all of the 
 * numbers from 1 to 20?
 * 
 * Evenly divisible: divisible with no remainder
 */

use std::io;

fn main () {
  println!("\nThis program finds the smallest possible number evenly divisible by two given numbers.");
  println!("\nWhat is your smallest number?");
  
  let mut smallest_divisor = String::new();
  io::stdin()
    .read_line(&mut smallest_divisor)
    .expect("Failed to read line.");
  println!("\nWhat is your largest number?");
  
  let mut largest_divisor = String::new();
  io::stdin()
    .read_line(&mut largest_divisor)
    .expect("Failed to read line.");

  println!("\nAttempting to find the smallest positive number...");

  let _largest_divisor = largest_divisor.trim().parse::<u32>().unwrap();
  let _smallest_divisor = smallest_divisor.trim().parse::<u32>().unwrap();

  // Starting from the largest divisor, start testing divisors
  let mut test = _largest_divisor;
  let mut found_number = false;
  while !found_number {
    let mut i = _smallest_divisor;
    while i <= _largest_divisor {
      let can_divide = test % i == 0;
      if !can_divide {
        // Move on to the next number in the test
        found_number = false;
        break;
      } else {
        found_number = true;
      }
      i = i + 1;
    }
    if !found_number {
      test = test + 1;
    }
  }
  println!("\nThe smallest positive number that is evenly divisble by all numbers between {} and {} is {}.\n", _smallest_divisor, _largest_divisor, test);
}