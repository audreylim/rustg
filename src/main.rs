use std::env;
use std::io::{Read, BufRead};
use std::fs::File;
use std::path::Path;

fn main() {
  let args: Vec<_> = env::args().collect();
  let pattern = args[1].as_slice();

    if let Some(f) = env::args().nth(2) {
      let mut s = String::new();
      read_file(f, &mut s);
      let r = s.as_slice();

      for line in r.lines() {
        if line.contains(pattern) {
          println!("{}", line);
        }
      }
    } else {
      let stdin = std::io::stdin();
      let lock = stdin.lock();

      for line in lock.lines() {
        let l = line.unwrap();
        if l.contains(pattern) {
          println!("{}", l);
        }
      }
    }
}

fn read_file(f: String, s: &mut String) {
  let path = Path::new(f.as_slice());
  let mut file_content = File::open(&path).unwrap();
  file_content.read_to_string(s);
}
