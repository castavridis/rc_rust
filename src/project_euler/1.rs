/* 
 * https://projecteuler.net/problem=1
 * 
 * If we list all the natural numbers below 10 that are multiples of 3 or 5, we 
 * get 3, 5, 6 and 9. The sum of these multiples is 23. Find the sum of all the 
 * multiples of 3 or 5 below 1000.
 * 
 **/
use std::io;

fn is_multiple(values: &Vec<&str>, number: i32) -> bool {
  let mut is_multiple: bool = false;
  for value in values {
    let _value: i32 = value.trim().parse::<i32>().unwrap();
    if number % _value == 0 {
      is_multiple = true;
    }
  }
  return is_multiple;
}

fn sum_multiples(values: &Vec<&str>, max: i32) {
  let mut sum: i32 = 0;

  // Iterate over an exclusive range up to but excluding `max`
  for step in 1..max {
    if is_multiple(values, step) {
      sum += step;
    }
  }
  
  // ! identifies this println! call as a call to a macro
  println!("\nSum: {sum}");
}

fn main () {
  println!("\nThis program will sum the multiples of a list of numbers between 1 and 1000.");

  println!("Please input the multiples you'd like to test, separated by a comma ',':\n");

  let mut input = String::new();
  io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line.");

  let multiples: Vec<&str> = input.split(",").collect();
  sum_multiples(&multiples, 1000)
}
