/** 
 * The following spiritual principles are the bedrock on which creative
 * recovery and discovery can be built. Read them through once a day, and
 * keep an inner ear cocked for any shifts in attitudes or beliefs.
 * – The Artist's Way, page 17
 */

use std::io;
use std::time::{ Duration };
use std::thread::sleep;

static BASIC_PRINCIPLES: [&'static str; 10] = [
  "Creativity is the natural order of life. Life is energy: pure creative energy.",
  "There is an underlying, in-dwelling creative force infusing all of life—including ourselves.",
  "When we open ourselves to our creativity, we open ourselves to the creator's creativity within us and our lives.",
  "We are, ourselves, creations. And we, in turn, are meant to continue creativity by being creative ourselves.",
  "Creativity is God's gift to us. Using our creativity is our gift back to God.",
  "The refusal to be creative is self-will and is counter to our true nature.",
  "When we open ourselves to exploring our creativity, we open ourselves to God: good orderly direction.",
  "As we open our creative channel to the creator, many gentle but powerful changes are to be expected.",
  "It is safe to open ourselves up to greater and greater creativity.",
  "Our creative dreams and yearnings come from a divine source. As we move toward our dreams, we move toward our divinity."
];

fn cycle_principles (step: usize) {
  if step == 1 {
    println!("\nPress ENTER to advance the principles.\n");
    sleep(Duration::new(1, 0))
  }
  let index = step - 1;
  let principle = BASIC_PRINCIPLES[index];
  println!("\t\x1b[33m {step}: {}\x1b[m", principle);
  if step == BASIC_PRINCIPLES.len() {
    println!("\nAll done.");
    return
  }
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Failed to read line.");
  if input.len() > 0 && input.trim().len() == 0 {
    let next_step = step + 1;
    cycle_principles(next_step);
  }
}

fn introduction () {

}

fn print_principles () {
  println!("\nAre you ready to read the Basic Principles? Y/N");
  
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Failed to read line.");
  let formatted_input = input.trim().to_lowercase();
  if formatted_input == "y" {
    cycle_principles(1);
  } else if formatted_input == "n" {
    println!("\nOkay, run me again when you're ready.");
  } else {
    println!("\nYou entered '{formatted_input}'. Please enter 'y' or 'n'. Let's try again.");
    print_principles();
  }
}

fn main () {
  introduction();
  print_principles();
}