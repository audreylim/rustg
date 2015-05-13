use std::env;
use std::io::{Read, BufRead};
use std::fs::File;
use std::path::Path;

extern crate getopts;

fn main() {
  let args: Vec<_> = env::args().collect();

  let opts = [
    getopts::optflag("i", "i", "case insensitive"),
    // add other flag options here.
  ];

  let matches = match getopts::getopts(args.tail(), &opts) {
    Ok(m) => m,
    Err(f) => {
      println!("{}", f);
      env::set_exit_status(1);
      return;
    }
  };

  if matches.opt_present("i") {
    // TODO: turn content into lowercase.
  }

  if !matches.free.is_empty() {
    let pattern = &matches.free[0];

    let mut files = matches.free.clone();
    files.swap_remove(0); // pop first argument from vector which is a pattern and not a file. 
    if files.len() > 0 {
      for f in files.iter() {
        let mut s = String::new();
        read_file(f, &mut s);
        let r = &s;

        for line in r.lines() {
          if line.contains(pattern) {
            if files.len() > 1 {
              println!("{}:{}", f, line);
            } else {
              println!("{}", line);
            }
          }
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
  } else {
    println!("no arguments provided");
  };
}

fn read_file(f: &String, s: &mut String) {
  let path = Path::new(&f);
  // TODO: Handle error messages properly.
  let mut file_content = File::open(&path).unwrap();
  file_content.read_to_string(s);
}
