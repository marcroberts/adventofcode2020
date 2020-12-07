use std::env;
use std::fs;
use regex::Regex;


fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];

  let content = fs::read_to_string(filename).unwrap();

  let mut valids: u16 = 0;

  let re_hcl = Regex::new(r"^\#[\da-f]{6}$").unwrap();
  let re_hgt = Regex::new(r"^\d+(cm|in)$").unwrap();
  let re_ecl = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
  let re_pid = Regex::new(r"^\d{9}$").unwrap();

  for passport_string in content.split("\n\n") {
    let one_line = passport_string.replace("\n", " ");
    // println!("{}", one_line);

    let mut keys_count = 0;
    let mut has_cid = false;
    let mut valid_data = true;

    for field in one_line.split(" ") {
      if field.len() == 0 {
        continue;
      }

      let key = &field[0..3];

      keys_count += 1;
      if key == "cid" {
        has_cid = true;
      }

      // part 2 only, reject after first failed check
      if true && valid_data {

        let value = &field[4..];

        if key == "byr" || key == "iyr" || key == "eyr" {
          let year: u16 = value.parse().unwrap();

          if (key == "byr" && (year < 1920 || year > 2002))
            || (key == "iyr" && (year < 2010 || year > 2020))
            || (key == "eyr" && (year < 2020 || year > 2030)) {
              valid_data = false;
            }
        }

        if key == "hgt" {
          if !re_hgt.is_match(value) {
            valid_data = false;
          } else {
            let len = value.len();
            let height: u16 = (&value[0..(len-2)]).parse().unwrap();
            let height_type = &value[(len-2)..len];

            if (height_type == "in" && (height < 59 || height > 76))
              || (height_type == "cm" && (height < 150 || height > 193)) {
                valid_data = false;
              }
          }
        }

        if key == "hcl" {
          if !re_hcl.is_match(value) {
            valid_data = false;
          }
        }

        if key == "ecl" {
          if !re_ecl.is_match(value) {
            valid_data = false;
          }
        }

        if key == "pid" {
          if !re_pid.is_match(value) {
            valid_data = false;
          }
        }

        if !valid_data {
          // println!("Rejected because of {}", key);
        }
      }
    }

    // println!("keys = {}", keys_count);

    if valid_data && (keys_count == 8 || keys_count == 7 && !has_cid) {
      valids += 1;
      // println!("Valid");
    } else {
      // println!("{}", one_line);
      // println!("NOT Valid");
    }
  }

  println!("There are {} valid passports", valids);
}
