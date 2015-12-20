extern crate getopts;
extern crate rosalind;

use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::env;

use rosalind::count_dna_nucleotides;
use getopts::Options;

fn do_work(data_file: &str) {
  let path = Path::new(data_file);
  let file_path = path.display();

  let mut file = match File::open(&path) {
    Ok(file) => file,
    Err(err) => panic!("couldn't open file {}: {}", file_path, err),
  };

  let mut s = String::new();
  match file.read_to_string(&mut s) {
    Ok(_) => (),
    Err(err) => panic!("couldn't read file {}: {}", file_path, err),
  }

  match count_dna_nucleotides(&s) {
    Ok(dna_nucleotides) => println!("Result: {}", dna_nucleotides),
    Err(err) => println!("{:?}", err),
  }
}

fn print_usage(program: &str, opts: Options) {
  let brief = format!("Usage: {} FILE [options]", program);
  print!("{}", opts.usage(&brief));
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();

  let mut opts = Options::new();
  opts.reqopt("d", "", "set data file name", "NAME");
  opts.optflag("h", "help", "print this help menu");
  let matches = match opts.parse(&args[1..]) {
    Ok(m) => m,
    Err(f) => panic!(f.to_string()),
  };

  if matches.opt_present("h") {
    return print_usage(&program, opts);
  }

  let data_file = matches.opt_str("d").unwrap();

  do_work(&data_file);
}
