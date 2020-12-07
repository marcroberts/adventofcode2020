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
      let min: u8 = (&cap[1]).to_string().parse().unwrap();
      let max: u8 = (&cap[2]).to_string().parse().unwrap();

      let ch: char = cap[3].as_bytes()[0] as char;
      let password: String = (&cap[4]).to_string();

      let mut char_count: u8 = 0;

      for c in password.chars() {
        if c == ch {
          char_count += 1;
        }
      }

      if char_count >= min && char_count <= max {
        valids += 1;
      }
    }
    
  }
  
  println!("Valid count = {}", valids);

}
