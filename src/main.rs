extern crate getopts;
extern crate rosalind;

use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::env;

use rosalind::dna::count_dna_nucleotides;
use rosalind::rna::transcribe_dna_into_rna;
use rosalind::revc::reverse_complement_dna;
use rosalind::fib::recurrence_relation;
use rosalind::prot::translate_rna_into_protein;
use getopts::{Options, Matches};

fn read_data_file(data_file: &str) -> String {
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

  return s;
}

fn do_task(matches: &Matches) {
  if matches.opt_str("t").is_none() {
    panic!("task name required");
  }

  let task: &str = &(matches.opt_str("t").unwrap());
  match task {
    "dna" => {
      if matches.opt_str("d").is_none() { panic!("data file required") }
      let data_file = matches.opt_str("d").unwrap();
      let s = read_data_file(&data_file);
      match count_dna_nucleotides(&s) {
        Ok(dna_nucleotides) => println!("Result: {}", dna_nucleotides),
        Err(err) => println!("{:?}", err),
      }
    },
    "rna" => {
      if matches.opt_str("d").is_none() { panic!("data file required") }
      let data_file = matches.opt_str("d").unwrap();
      let s = read_data_file(&data_file);
      match transcribe_dna_into_rna(&s) {
        Ok(rna) => println!("Result: {}", rna),
        Err(err) => println!("{:?}", err),
      }
    },
    "revc" => {
      if matches.opt_str("d").is_none() { panic!("data file required") }
      let data_file = matches.opt_str("d").unwrap();
      let s = read_data_file(&data_file);
      match reverse_complement_dna(&s) {
        Ok(revc) => println!("Result: {}", revc),
        Err(err) => println!("{:?}", err),
      }
    },
    "fib" => {
      if matches.opt_str("n").is_none() { panic!("N parameter required") }
      if matches.opt_str("k").is_none() { panic!("K parameter required") }
      let n: u8 = matches.opt_str("n").unwrap().parse::<u8>().unwrap();
      let k: u8 = matches.opt_str("k").unwrap().parse::<u8>().unwrap();
      let fib = recurrence_relation(n, k);
      println!("Result: {}", fib);
    },
    "prot" => {
      if matches.opt_str("d").is_none() { panic!("data file required") }
      let data_file = matches.opt_str("d").unwrap();
      let s = read_data_file(&data_file);
      match translate_rna_into_protein(&s) {
        Ok(prot) => println!("Result: {}", prot),
        Err(err) => println!("{:?}", err),
      }
    }
    _ => println!("Unknown task: {}", task),
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
  opts.optopt("d", "data", "set data file name", "NAME");
  opts.optflag("h", "help", "print this help menu");
  opts.optopt("k", "", "k value for fibonacci", "K");
  opts.optopt("n", "", "n value for fibonacci", "N");
  opts.optopt("t", "task", "provide task name", "dna|rna|revc|fib|prot");
  let matches = match opts.parse(&args[1..]) {
    Ok(m) => m,
    Err(f) => panic!(f.to_string()),
  };

  if matches.opt_present("h") {
    return print_usage(&program, opts);
  }

  do_task(&matches);
}
