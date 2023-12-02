#![deny(warnings)]

mod args;

use std::io::{BufWriter, stdout, Write};
use ripgen_lib::{RipGenIterator, RipGenManager};
use crate::args::Args;
use clap::Parser;

const FAST_MODE_WORDLIST_LEN: usize = 10;
const DEFAULT_WORD_LEN: usize = 5;

fn main() {
    let args: Args = Args::parse();

    let domains = args.get_domain_str()
        .expect("Failed to read in domains.");
    let wordlist = args.get_wordlist_str()
        .expect("Failed to read in wordlist file.");
    let word_len = args.min_word_len.unwrap_or(DEFAULT_WORD_LEN);

    let wordlist_lines = get_wordlist(&wordlist, &args);

    let manager = RipGenManager::new(
        domains.lines(),
        wordlist_lines,
        &|word| word.len() >= word_len
    ).expect("Failed to create ripgen iterator");

    let rip_iter = manager
        .transform(ripgen_lib::dnsgen::swap_word_transform)
        .chain_transform(ripgen_lib::dnsgen::permute_words_transform)
        .chain_transform(ripgen_lib::dnsgen::numbers_transform)
        .chain_transform(ripgen_lib::dnsgen::dash_transform);

    stream_output(rip_iter);
}

fn stream_output(rip_iter: impl Iterator<Item = String>) {
    let out = stdout();
    let stdout_lock = out.lock();
    let mut buf = BufWriter::new(stdout_lock);

    for line in rip_iter {
        if writeln!(buf, "{}", line).is_err() {
            // user might be using `head` to only grab the first couple of entries - we should exit
            let _ = buf.flush();
            return;
        }
    }

    let _ = buf.flush();
}

fn get_wordlist<'a>(wordlist: &'a str, args: &Args) -> impl Iterator<Item = &'a str> {
    // https://github.com/ProjectAnte/dnsgen/blob/16daeef81205e7663708b3ee11d759215c7168fe/dnsgen/dnsgen.py#L220
    let mut wordlist_iter = None;
    let mut fast_wordlist_iter = None;

    match args.fast {
        Some(fast_mode) if fast_mode => {
            fast_wordlist_iter = Some(wordlist.lines().take(FAST_MODE_WORDLIST_LEN))
        },
        _ => wordlist_iter = Some(wordlist.lines())
    }

    wordlist_iter
        .into_iter()
        .flatten()
        .chain(fast_wordlist_iter.into_iter().flatten())
}