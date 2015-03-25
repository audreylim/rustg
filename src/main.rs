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
    println!("a");
  }

  let pattern = if !matches.free.is_empty() {
    let p = matches.free[0].as_slice();
    p  
  } else {
    println!("no arguments provided");
  };

  println!("{}", pattern);

//
//  let pattern = args[1].as_slice();
//
//    if let some(f) = env::args().nth(2) {
//      let mut s = string::new();
//      read_file(f, &mut s);
//      let r = s.as_slice();
//
//      for line in r.lines() {
//        if line.contains(pattern) {
//          println!("{}", line);
//        }
//      }
//    } else {
//      let stdin = std::io::stdin();
//      let lock = stdin.lock();
//
//      for line in lock.lines() {
//        let l = line.unwrap();
//        if l.contains(pattern) {
//          println!("{}", l);
//        }
//      }
//    }
}
//
//fn read_file(f: String, s: &mut String) {
//  let path = Path::new(f.as_slice());
//  let mut file_content = File::open(&path).unwrap();
//  file_content.read_to_string(s);
//}
