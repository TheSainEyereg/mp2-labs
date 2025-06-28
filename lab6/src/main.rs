mod substring;

use substring::{boyer_moore, kmp, rabin_karp};

fn main() {
    let text = "ABACABABACB";
    let pattern = "ABAC";

    println!("Rabin-Karp: {:?}", rabin_karp(text, pattern));
    println!("Boyer-Moore: {:?}", boyer_moore(text, pattern));
    println!("KMP: {:?}", kmp(text, pattern));
}
