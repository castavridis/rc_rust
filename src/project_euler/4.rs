/**
 * https://projecteuler.net/problem=4
 * 
 * A palindromic number reads the same both ways. 
 * The largest palindrome made from the product of two 2-digit numbers is 
 * 9009 = 91 x 99. Find the largest palindrome made from the product of 
 * two 3-digit numbers.
 */

fn is_palindrome (test: u32) -> bool {
  if test < 100000 {
    return false;
  }
  let string = test.to_string();
  let start = &string[..3];
  let end = format!("{}{}{}", &string[5..6], &string[4..5], &string[3..4]);
  if start == end {
    return true;
  }
  return false;
}

fn main () {
  let mut x: u32 = 999;
  let mut y: u32 = 999;
  while x > 99 {
    while y > 99 {
      if is_palindrome(x*y) {
        println!("{} * {} = {}", x, y, x*y);
        break;
      }
      y = y - 1;
    }
    x = x - 1;
    y = 999;
  }
}