use std::env;
use std::io;
use std::io::BufRead;
use std::fs::File;
use regex::Regex;

fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];

	let file = File::open(filename).unwrap();
  let reader = io::BufReader::new(file);
  
  let re = Regex::new(r"(\d+)-(\d+) (.): (.*)").unwrap();

  let mut valids: u16 = 0;

	// load the values
	for line in reader.lines() {
    let str = line.unwrap();

    for cap in re.captures(&str) {
      // println!("min {}, max {}, char {}, password {}", &cap[1], &cap[2], &cap[3], &cap[4]);
      let a: usize = (&cap[1]).to_string().parse().unwrap();
      let b: usize = (&cap[2]).to_string().parse().unwrap();

      let ch: char = cap[3].as_bytes()[0] as char;

      let char_a: char = cap[4].as_bytes()[a-1] as char;
      let char_b: char = cap[4].as_bytes()[b-1] as char;

      let mut correct_chars_count = 0;

      if char_a == ch {
        correct_chars_count += 1 ;
      }

      if char_b == ch {
        correct_chars_count += 1 ;
      }

      if correct_chars_count == 1 {
        valids += 1;
      }
    }
    
  }
  
  println!("Valid count = {}", valids);

}
