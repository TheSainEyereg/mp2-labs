mod substring;

use substring::{boyer_moore, kmp, rabin_karp};

fn main() {
    let text = "ABABAC";
    let pattern = "ABA";

    println!("Rabin-Karp: {:?}", rabin_karp(text, pattern));
    println!("KMP: {:?}", kmp(text, pattern));
    println!("Boyer-Moore: {:?}", boyer_moore(text, pattern));

    // Вопросы:
    // 1. Эвристика
    // 2. Стоп символ хорошего суф
    // 3. Как работатет без таблицы перехода
    // 4. КМП: алг префикса, функ без табл (изначальная логика)
}
