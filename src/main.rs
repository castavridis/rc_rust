// JS: mport say from 'ferris_says'
use ferris_says::say;

// JS: import {stdout, BufWriter} from 'std:io' 
use std::io::{stdout, BufWriter};

/** 
 * Using VSCode and the rust-analyzer extension, types are automatically 
 * inferred and the IDE inserts 'inlays' for the types into the code
 * */ 

// JS: function main () {}
fn main() {
    let stdout = stdout();
    let message = String::from(" 🦀\nHellewww\n 🦀\n 🐙");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}
