mod stack;
mod symbol;

use crate::stack::Stack;
use crate::symbol::Symbol;

use globwalk::*;
use rayon::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::result::Result;
use std::time::{Instant, SystemTime};

const PATTERNS: &'static [&'static str] =
    &["*.{vw,rv,sl,dg,src,dd,pkg,mod,cls,cls,bpo,rpt,mnu,cal,con}"];

fn main() {
    println!("{} cores", num_cpus::get());
    println!("{} cpus", num_cpus::get_physical());

    let base_dir = "/Users/KT/Documents/HeavyBid";
    build_index(base_dir);
}

fn build_index(base_dir: &str) {
    let now = Instant::now();
    let tree = sled::open("temp/db").unwrap();

    GlobWalkerBuilder::from_patterns(base_dir, PATTERNS)
        .case_insensitive(true)
        .build()
        .unwrap()
        .into_iter()
        .filter_map(Result::ok)
        .collect::<Vec<DirEntry>>()
        .par_iter()
        .for_each(move |e| index_file(e, &tree));

    println!("{:?}", now.elapsed())
}

fn index_file(dir_entry: &DirEntry, tree: &sled::Db) {
    if let Some(path) = dir_entry.path().to_str() {
        parse_file(path, &tree);
    } else {
        println!(
            "{:?} - Unable to parse file {:?}",
            SystemTime::now(),
            dir_entry.path()
        );
    }
    let _ = tree.flush();
}

fn parse_file(file_path: &str, tree: &sled::Db) {
    let file = File::open(file_path).unwrap();

    let mut containers: Stack<Symbol> = stack::Stack::new();

    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut len = reader.read_line(&mut line).unwrap();
    while len > 0 {
        len = reader.read_line(&mut line).unwrap();
    }
    println!("finish - {}", file_path);
    // TODO: get hash of file as u32
    // match tree.insert(file_path, b"soemthing") {
    //     Ok(_) => {}
    //     Err(e) => println!("{}", e),
    // };

    // create stacks
    // read file lines
    // ...`
    // return symbols
}
