extern crate getopts;
extern crate rosalind;

use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::env;

use rosalind::dna::count_dna_nucleotides;
use rosalind::rna::transcribe_dna_into_rna;
use rosalind::revc::reverse_complement_dna;
use rosalind::fib::{recurrence_relation, recurrence_relation_with_stop};
use rosalind::prot::translate_rna_into_protein;
use rosalind::hamm::hamming_distance;
use rosalind::subs::motif_lookup;
use rosalind::gc::best_gc_content_in_dataset;
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
  if matches.opt_str("t").is_none() { panic!("task name required"); }

  let task: &str = &(matches.opt_str("t").unwrap());
  let data_file: String;
  let mut file_content = String::new();

  match task {
    "fib" | "fibd" => (),
    _ => {
      if matches.opt_str("d").is_none() { panic!("data file required") }
      data_file = matches.opt_str("d").unwrap();
      file_content = read_data_file(&data_file);
    }
  }

  match task {
    "dna" => {
      match count_dna_nucleotides(&file_content) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("{:?}", err),
      }
    },
    "rna" => {
      match transcribe_dna_into_rna(&file_content) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("{:?}", err),
      }
    },
    "revc" => {
      match reverse_complement_dna(&file_content) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("{:?}", err),
      }
    },
    "fib" => {
      if matches.opt_str("n").is_none() { panic!("month amount to calculate population required (n)") }
      if matches.opt_str("k").is_none() { panic!("offspring amount from each pair required (k)") }
      let n: usize = matches.opt_str("n").unwrap().parse::<usize>().unwrap();
      let k: usize = matches.opt_str("k").unwrap().parse::<usize>().unwrap();
      match recurrence_relation(n, k) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("{:?}", err),
      }
    },
    "fibd" => {
      if matches.opt_str("n").is_none() { panic!("month amount to calculate population required (n)") }
      if matches.opt_str("m").is_none() { panic!("lifetime in months required (m)") }
      let n: usize = matches.opt_str("n").unwrap().parse::<usize>().unwrap();
      let m: usize = matches.opt_str("m").unwrap().parse::<usize>().unwrap();
      match recurrence_relation_with_stop(n, m) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("{:?}", err),
      }
    },
    "prot" => {
      match translate_rna_into_protein(&file_content) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("{:?}", err),
      }
    },
    "hamm" => {
      let mut lines = file_content.lines();
      let s = lines.next().unwrap();
      let t = lines.next().unwrap();
      match hamming_distance(&s, &t) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("{:?}", err),
      }
    },
    "subs" => {
      let mut lines = file_content.lines();
      let s = lines.next().unwrap();
      let t = lines.next().unwrap().trim();
      match motif_lookup(&s, &t) {
        Ok(result) => println!("Result: {:?}", result),
        Err(err) => println!("{:?}", err),
      }
    },
    "gc" => {
      match best_gc_content_in_dataset(&file_content) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("{:?}", err),
      }
    },
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
  let supported_tasks = "dna|rna|revc|fib|fibd|prot|hamm|subs|gc";
  opts.optopt("d", "data", "set data file name", "NAME");
  opts.optflag("h", "help", "print this help menu");
  opts.optopt("k", "", "offspring amount from each pair", "K");
  opts.optopt("m", "", "lifetime in months", "M");
  opts.optopt("n", "", "month amount to calculate population", "N");
  opts.optopt("t", "task", "task name", supported_tasks);
  let matches = match opts.parse(&args[1..]) {
    Ok(m) => m,
    Err(f) => panic!(f.to_string()),
  };

  if matches.opt_present("h") {
    return print_usage(&program, opts);
  }

  do_task(&matches);
}
