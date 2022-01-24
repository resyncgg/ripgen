mod args;

use std::io::{BufWriter, stdout, Write};
use ripgen_lib::{RipGenIterator, RipGenManager};
use crate::args::Args;
use clap::Parser;

const DEFAULT_WORD_LEN: usize = 5;

fn main() {
    let args: Args = Args::parse();

    let domains = args.get_domain_str()
        .expect("Failed to read in domains.");
    let wordlist = args.get_wordlist_str()
        .expect("Failed to read in wordlist file.");
    let word_len = args.min_word_len.unwrap_or(DEFAULT_WORD_LEN);

    let manager = RipGenManager::new(
        domains.lines(),
        wordlist.lines(),
        &|word| word.len() >= word_len
    ).expect("Failed to create Ripgen iterator");

    let rip_iter = manager
        .transform(ripgen_lib::dnsgen::swap_word_transform)
        .chain_transform(ripgen_lib::dnsgen::permute_words_transform)
        .chain_transform(ripgen_lib::dnsgen::numbers_transform)
        .chain_transform(ripgen_lib::dnsgen::dash_transform);

    if args.streaming {
        stream_output(rip_iter);
    } else {
        dump_output(rip_iter);
    }
}

fn stream_output(rip_iter: impl Iterator<Item = String>) {
    let out = stdout();
    let stdout_lock = out.lock();
    let mut buf = BufWriter::new(stdout_lock);

    for line in rip_iter {
        writeln!(buf, "{}", line).expect("Failed to write to stdout buffer.");
    }

    buf.flush().expect("Failed to perform final flush to stdout buffer.");
}

fn dump_output(rip_iter: impl Iterator<Item = String>) {
    let output = rip_iter.collect::<Vec<String>>().join("\n");

    println!("{}", output);
}