/** 
 * Note: The colors in this terminal are based on monokai pro (filter spectrum)
 * 
 * The following spiritual principles are the bedrock on which creative
 * recovery and discovery can be built. Read them through once a day, and
 * keep an inner ear cocked for any shifts in attitudes or beliefs.
 * – The Artist's Way, page 17
 * 
 */

use std::io;
use std::time::{ Duration };
use std::thread::sleep;

static NUM_LINES: i32 = 9;
static NUM_COLOR_FRAMES: usize = 7;
static TEXT_COLORS: [(i32,i32,i32); NUM_COLOR_FRAMES] = [
  (230, 98, 207),
  (215, 120, 175),
  (200, 142, 143),
  (185, 164, 111),
  (170, 186, 79),
  (155, 208, 47),
  (142, 232, 14),
];
static BASE_COLOR: (i32,i32,i32) = (43,43,43);
static BASIC_PRINCIPLES: [&'static str; 10] = [
  "Creativity is the natural order of life.\nLife is energy: pure creative energy.",
  "There is an underlying, in-dwelling creative\nforce infusing all of life—including ourselves.",
  "When we open ourselves to our creativity, we open ourselves\nto the creator's creativity within us and our lives.",
  "We are, ourselves, creations. And we, in turn, are meant\nto continue creativity by being creative ourselves.",
  "Creativity is God's gift to us.\nUsing our creativity is our gift back to God.",
  "The refusal to be creative is self-will\nand is counter to our true nature.",
  "When we open ourselves to exploring our creativity,\nwe open ourselves to God: good orderly direction.",
  "As we open our creative channel to the creator,\nmany gentle but powerful changes are to be expected.",
  "It is safe to open ourselves\nup to greater and greater creativity.",
  "Our creative dreams and yearnings come from a divine source.\nAs we move toward our dreams, we move toward our divinity."
];
static PRINCIPLE_NUMS: [&'static str; 10] = [
  "𝖔𝖓𝖊",
  "𝖙𝖜𝖔",
  "𝖙𝖍𝖗𝖊𝖊",
  "𝖋𝖔𝖚𝖗",
  "𝖋𝖎𝖛𝖊",
  "𝖘𝖎𝖝",
  "𝖘𝖊𝖛𝖊𝖓",
  "𝖊𝖎𝖌𝖍𝖙",
  "𝖓𝖎𝖓𝖊",
  "𝖙𝖊𝖓",
];
const SCENE_LENGTH_MS: u64 = 2000;
const SCENE_STEPS: u64 = 7;
const ANIMATION_DURATION: u64 = SCENE_LENGTH_MS/SCENE_STEPS;

fn cycle_principles (step: usize) {
  if step == 1 {
    // Print ten empty lines
    for _ in 0..(NUM_LINES-1) {
      println!();
    }
    println!("\x1b[38;2;{};{};{}m𝙿𝚛𝚎𝚜𝚜 𝙴𝙽𝚃𝙴𝚁 𝚝𝚘 𝚊𝚍𝚟𝚊𝚗𝚌𝚎 𝚝𝚑𝚎 𝚙𝚛𝚒𝚗𝚌𝚒𝚙𝚕𝚎𝚜.\x1b[m",
      BASE_COLOR.0,
      BASE_COLOR.1,
      BASE_COLOR.2,
    );

    // Move cursor up to be above "Press ENTER...""
    print!("\x1b[{}A", 1);
    
    sleep(Duration::new(1, 0))
  }
  let index = step - 1;
  let principle = BASIC_PRINCIPLES[index];
  let principle_num: &str = PRINCIPLE_NUMS[index];
  let mut input = String::new();
  let mut iteration = 0;
  
  loop {
    let color1:(i32,i32,i32) = TEXT_COLORS[iteration % NUM_COLOR_FRAMES];
    let color2:(i32,i32,i32) = TEXT_COLORS[NUM_COLOR_FRAMES - 1 - (iteration % NUM_COLOR_FRAMES)];
    let lines: Vec<&_> = principle.split('\n').collect::<Vec<_>>();
    let line1: &str = lines[0];
    let line2: &str = lines[1];

    // Move cursor up to the start of our animation area
    print!("\x1b[{}A", NUM_LINES);

    for line in 0..NUM_LINES {
      // Clear the line
      print!("\r\x1b[K");

      if line == 3 {
        print!("\x1b[38;2;{};{};{}m\t𝔓𝔯𝔦𝔫𝔠𝔦𝔭𝔩𝔢 {principle_num}:\x1b[m",
          BASE_COLOR.0,
          BASE_COLOR.1,
          BASE_COLOR.2,
        );
      }
      if line == 4 {
        print!("\x1b[38;2;{};{};{}m\t  {}\x1b[m",
          color1.0, color1.1, color1.2,
          line1,
        );
      }
      if line == 5 {
        print!("\x1b[38;2;{};{};{}m\t    {}\x1b[m",
          color2.0, color2.1, color2.2,
          line2,
        );
      }

       // move to the next line
      println!();
    }
    
    sleep(Duration::from_millis(ANIMATION_DURATION));

    iteration += 1;

    if iteration >= 9 {
      io::stdin().read_line(&mut input).expect("Failed to read line.");
      if input.len() > 0 && input.trim().len() == 0 && step != 0 {
        let next_step = step + 1;
        if next_step <= BASIC_PRINCIPLES.len() {
          // Move cursor up one to compensate for the ENTER
          print!("\x1b[{}A", 1);
          cycle_principles(next_step);
        } else {
          println!("\n𝙰𝚕𝚕 𝚍𝚘𝚗𝚎.\n\n\n");
          return
        }
        break;
      }
    }
  }
}

fn print_principles () {
  println!("\x1b[38;2;{};{};{}m\n𝙰𝚛𝚎 𝚢𝚘𝚞 𝚛𝚎𝚊𝚍𝚢 𝚝𝚘 𝚛𝚎𝚊𝚍 𝚝𝚑𝚎 𝙱𝚊𝚜𝚒𝚌 𝙿𝚛𝚒𝚗𝚌𝚒𝚙𝚕𝚎𝚜? (𝚈)/𝙽\x1b[m",
      BASE_COLOR.0,
      BASE_COLOR.1,
      BASE_COLOR.2,
    );
  
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Failed to read line.");
  let formatted_input = input.trim().to_lowercase();
  if formatted_input == "y" || formatted_input.len() == 0 {
    cycle_principles(1);
  } else if formatted_input == "n" {
    println!("\n𝙾𝚔𝚊𝚢, 𝚟𝚒𝚜𝚒𝚝 𝚖𝚎 𝚊𝚐𝚊𝚒𝚗 𝚠𝚑𝚎𝚗 𝚢𝚘𝚞'𝚛𝚎 𝚛𝚎𝚊𝚍𝚢.");
  } else {
    println!("\n𝚈𝚘𝚞 𝚎𝚗𝚝𝚎𝚛𝚎𝚍 '{formatted_input}'. 𝙿𝚕𝚎𝚊𝚜𝚎 𝚎𝚗𝚝𝚎𝚛 '𝚢' 𝚘𝚛 '𝚗'. 𝙻𝚎𝚝'𝚜 𝚝𝚛𝚢 𝚊𝚐𝚊𝚒𝚗.");
    print_principles();
  }
}

fn main () {
  print_principles();
}