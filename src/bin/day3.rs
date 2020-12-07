use std::env;
use std::io::{BufRead, BufReader};
use std::io::{Seek, SeekFrom};
use std::fs::File;

fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];

  let mut file = File::open(filename).unwrap();  
  let mut product: u64 = 1;

  let mut trees_count = trees_for_slope(1, 1, &mut file);
  product *= trees_count as u64;
  println!("Trees count for (Right: {}, Down: {}) = {}", 1, 1, trees_count);

  trees_count = trees_for_slope(3, 1, &mut file);
  product *= trees_count as u64;
  println!("Trees count for (Right: {}, Down: {}) = {}", 3, 1, trees_count);

  trees_count = trees_for_slope(5, 1, &mut file);
  product *= trees_count as u64;
  println!("Trees count for (Right: {}, Down: {}) = {}", 5, 1, trees_count);

  trees_count = trees_for_slope(7, 1, &mut file);
  product *= trees_count as u64;
  println!("Trees count for (Right: {}, Down: {}) = {}", 7, 1, trees_count);

  trees_count = trees_for_slope(1, 2, &mut file);
  product *= trees_count as u64;
  println!("Trees count for (Right: {}, Down: {}) = {}", 1, 2, trees_count);

  println!("Product is {}", product);
}

fn trees_for_slope(right: usize, down: usize, file: &mut File) -> u16 {
  let mut trees_count: u16 = 0;

  let mut x: usize = 0;

  file.seek(SeekFrom::Start(0)).unwrap();
  let reader = BufReader::new(file);

  let lines = reader.lines();

  // println!("\n=====\n");

	// load the values
	for (index, line) in lines.enumerate() {
    let line_str = line.unwrap();
    // println!("{}", line_str);

    if index < down {
      continue;
    }

    // deal with 'down'
    if index % down != 0 {
      continue
    }

    x += right;


    let line_bytes = line_str.as_bytes();

    let line_length: usize = line_bytes.len();

    let position: usize = x % line_length;

    
    let thing: char = line_bytes[position] as char;
    // println!("{}\u{8}^{}", " ".repeat(position+1), thing);

    if thing == '#' {
      trees_count += 1;
    }

  }

  return trees_count;
}
