use std::{env, path::Path, fs::File, io::{BufReader, BufRead}};

use itertools::Itertools;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let f = File::open(filename).expect("No such file");
    let br = BufReader::new(f);
    br.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("\nusage: combos [FILE] [SIZE]\n");
        return;
    }
    let filename = &args[1];
    let k = &args[2].parse::<u64>().expect("Unable to parse size");
    let kk = *k as usize;
    let lines = lines_from_file(filename);
    let p = lines.into_iter().permutations(kk);
    p.into_iter().for_each(|f| println!("{}", f.concat()));
}
