//! `rosalind-cli` is the command line interaface for [rosalind](https://github.com/antklim/rosalind) crate
//! which contains solutions of problems from [Rosalind](http://rosalind.info/) site.
//!
//! # Command structure
//! ```
//! rosalind-cli [options]
//! ```
//!
//! # Options supported by `rosalind-cli`
//! ## General options
//! * `-h`, `--help` prints help menu
//! * `-t`, `--task` sets task name related to a particular problem
//!
//! ## Task specific options
//! * `-d`, `--data` sets path to data file
//! * `-k` offspring amount from each pair (used only in Fibonacci tasks)
//! * `-m` species lifetime in months (used only in Fibonacci tasks)
//! * `-n` month amount to calculate population size (used only in Fibonacci tasks)
//!
//! # Problem solutions provided by `rosalind-cli`
//! ### Counting DNA Nucleotides, task name `dna`
//! ```
//! rosalind-cli -t dna -d rosalind_dna.txt
//! ```
//!
//! ### Transcribing DNA into RNA, task name `rna`
//! ```
//! rosalind-cli -t rna -d rosalind_rna.txt
//! ```
//!
//! ### Complementing a Strand of DNA, task name `revc`
//! ```
//! rosalind-cli -t revc -d rosalind_revc.txt
//! ```
//!
//! ### Rabbits and Recurrence Relations, task name `fib`
//! ```
//! rosalind-cli -t fib -n 5 -k 3
//! ```
//!
//! ### Computing GC Content, task name `gc`
//! ```
//! rosalind-cli -t gc -d rosalind_gc.txt
//! ```
//!
//! ### Counting Point Mutations, task name `hamm`
//! ```
//! rosalind-cli -t hamm -d rosalind_hamm.txt
//! ```
//!
//! ### Mendel's First Law, task name `iprb`
//! ```
//! rosalind-cli -t iprb -d rosalind_iprb.txt
//! ```
//!
//! ### Translating RNA into Protein, task name `prot`
//! ```
//! rosalind-cli -t prot -d rosalind_prot.txt
//! ```
//!
//! ### Finding a Motif in DNA, task name `subs`
//! ```
//! rosalind-cli -t subs -d rosalind_subs.txt
//! ```
//!
//! ### Mortal Fibonacci Rabbits, task name `fibd`
//! ```
//! rosalind-cli -t fibd -n 6 -m 3
//! ```
//!
//! ### Inferring mRNA from Protein, task name `mrna`
//! ```
//! rosalind-cli -t mrna -d rosalind_mrna.txt
//! ```
//!
//! ### Calculating Protein Mass, task name `prtm`
//! ```
//! rosalind-cli -t prtm -d rosalind_prtm.txt
//! ```
//!
//! ### Calculating Consensus and Profile, task name `cons`
//! ```
//! rosalind-cli -t cons -d rosalind_cons.txt
//! ```

extern crate getopts;
extern crate rosalind;

use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::env;

use rosalind::dna::count_dna_nucleotides;
use rosalind::rna::transcribe_dna_into_rna;
use rosalind::revc::reverse_complement_dna;
use rosalind::fib::*;
use rosalind::prot::*;
use rosalind::hamm::hamming_distance;
use rosalind::subs::motif_lookup;
use rosalind::gc::best_gc_content_in_dataset;
use rosalind::iprb::dominant_allele_probability;
use rosalind::cons::{consensus, profile};
use rosalind::utils::parse_fasta_dataset;
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

fn prepare_task(matches: &Matches) -> (String, String, usize, usize, usize) {
    if matches.opt_str("t").is_none() { panic!("task name required"); }

    let task: String = matches.opt_str("t").unwrap();
    let data_file: String;
    let mut file_content = String::new();
    let mut n: usize = 0usize;
    let mut m: usize = 0usize;
    let mut k: usize = 0usize;

    match task.as_ref() {
        "fib" => {
            if matches.opt_str("n").is_none() {
                panic!("month amount to calculate population required (n)");
            }

            if matches.opt_str("k").is_none() {
                panic!("offspring amount from each pair required (k)");
            }

            n = matches.opt_str("n").unwrap().parse::<usize>().unwrap();
            k = matches.opt_str("k").unwrap().parse::<usize>().unwrap();
        }
        "fibd" => {
            if matches.opt_str("n").is_none() {
                panic!("month amount to calculate population required (n)");
            }

            if matches.opt_str("m").is_none() {
                panic!("lifetime in months required (m)");
            }

            n = matches.opt_str("n").unwrap().parse::<usize>().unwrap();
            m = matches.opt_str("m").unwrap().parse::<usize>().unwrap();
        },
        _ => {
            if matches.opt_str("d").is_none() { panic!("data file required") }
            data_file = matches.opt_str("d").unwrap();
            file_content = read_data_file(&data_file);
        }
    }

    (task, file_content, n, m, k)
}

fn do_task(task: &str, file_content: &str, n: usize, m: usize, k: usize) {
    match task {
        "dna" => {
            match count_dna_nucleotides(file_content) {
                Ok(result) => println!("Result: {}", result),
                Err(err) => println!("{:?}", err),
            }
        },
        "rna" => {
            match transcribe_dna_into_rna(file_content) {
                Ok(result) => println!("Result: {}", result),
                Err(err) => println!("{:?}", err),
            }
        },
        "revc" => {
            match reverse_complement_dna(file_content) {
                Ok(result) => println!("Result: {}", result),
                Err(err) => println!("{:?}", err),
            }
        },
        "fib" => {
            match recurrence_relation(n, k) {
                Ok(result) => println!("Result: {}", result),
                Err(err) => println!("{:?}", err),
            }
        },
        "fibd" => {
            match recurrence_relation_with_stop(n, m) {
                Ok(result) => println!("Result: {}", result),
                Err(err) => println!("{:?}", err),
            }
        },
        "prot" => {
            match translate_rna_into_protein(file_content) {
                Ok(result) => println!("Result: {}", result),
                Err(err) => println!("{:?}", err),
            }
        },
        "mrna" => {
            match get_number_of_rna_from_protein(file_content) {
                Ok(result) => println!("Result: {}", result),
                Err(err) => println!("{:?}", err),
            }
        },
        "prtm" => {
            match get_protein_mass(file_content) {
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
            match best_gc_content_in_dataset(file_content) {
                Ok(result) => println!("Result: {}", result),
                Err(err) => println!("{:?}", err),
            }
        },
        "iprb" => {
            let params: Vec<u8> = (file_content)
                .split_whitespace()
                .map(|x| x.parse::<u8>().unwrap())
                .collect::<Vec<u8>>();
            match dominant_allele_probability(params[0], params[1], params[2]) {
                Ok(result) => println!("Result: {}", result),
                Err(err) => println!("{:?}", err),
            }
        },
        "cons" => {
            let dataset = parse_fasta_dataset(file_content).unwrap();
            let ds: Vec<&str> = dataset.iter().map(|s| s.as_ref()).collect();
            match profile(ds) {
                Ok(profile) => {
                    println!("Profile:\n{}", profile);
                    match consensus(profile) {
                        Ok(consensus) => println!("Consensus:\n{}", consensus),
                        Err(err) => println!("{:?}", err),
                    }
                },
                Err(err) => println!("{:?}", err),
            }
        },
        _ => println!("Unknown task: {}", task),
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

#[cfg(test)]
mod tests {
    // use super::{do_task, prepare_task};
    // #[test]
    // fn prepare_task_should_panic_when_task_is_not_found() {
    // }

    // #[test]
    // fn prepare_task_should_panic_when_task_is_fib_and_n_not_found() {
    // }

    // #[test]
    // fn prepare_task_should_panic_when_task_is_fib_and_k_not_found() {
    // }

    // #[test]
    // fn prepare_task_should_panic_when_task_is_fibd_and_n_not_found() {
    // }

    // #[test]
    // fn prepare_task_should_panic_when_task_is_fibd_and_m_not_found() {
    // }

    // #[test]
    // fn prepare_task_should_panic_when_data_file_not_found() {
    // }

    // #[test]
    // fn do_task_should_print_message_when_unknown_task_received() {
    // }

    // #[test]
    // fn do_task_should_call_dna() {
    // }

    // #[test]
    // fn do_task_should_call_rna() {
    // }

    // #[test]
    // fn do_task_should_call_revc() {
    // }

    // #[test]
    // fn do_task_should_call_fib() {
    // }

    // #[test]
    // fn do_task_should_call_fibd() {
    // }

    // #[test]
    // fn do_task_should_call_prot() {
    // }

    // #[test]
    // fn do_task_should_call_mrna() {
    // }

    // #[test]
    // fn do_task_should_call_hamm() {
    // }

    // #[test]
    // fn do_task_should_call_subs() {
    // }

    // #[test]
    // fn do_task_should_call_gc() {
    // }

    // #[test]
    // fn do_task_should_call_iprb() {
    // }

    // #[test]
    // fn do_task_should_call_prtm() {
    // }

    // #[test]
    // fn do_task_should_call_cons() {
    // }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    let supported_tasks = "dna|rna|revc|fib|fibd|prot|hamm|subs|gc|mrna|iprb|prtm|cons";

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

    let (task, file_content, n, m, k) = prepare_task(&matches);
    do_task(&task, &file_content, n, m, k);
}
