/*
d — размер алфавита
q — некоторое простое
Rabin-Karp(T,P)
    n = len(T)
    m = len(P)
    d_m = d^(m-1) mod q
    h = h' = 0
    for i = 0 to m-1
        h = d*h + P[i] mod q
        h'= d*h'+ T[i] mod q
    for s = 0 to n-m
        if h == h' then
            if P == T[s..s+m-1] then
                s → result
            if s < n-m then
                h'= (d*(h'-d_m*T[s])+T[s+m]) mod q
*/
pub fn rabin_karp(text: &str, pattern: &str) -> Vec<usize> {
    let d: u64 = 256; // ASCII
    let q: u64 = 107; // Prime number
    let n = text.len();
    let m = pattern.len();

    let mut result = Vec::new();

    if m == 0 || m > n {
        return result;
    }

    let text_bytes = text.as_bytes();
    let pattern_bytes = pattern.as_bytes();

    let mut d_m = 1u64;
    for _ in 0..m - 1 {
        d_m = (d_m * d) % q; // d^(m-1) mod q
    }

    let mut h = 0u64; // P hash
    let mut h_t = 0u64; // T hash

    for i in 0..m {
        h = (d * h + pattern_bytes[i] as u64) % q;
        h_t = (d * h_t + text_bytes[i] as u64) % q;
    }

    for s in 0..=n - m {
        if h == h_t {
            if &text[s..s + m] == pattern {
                result.push(s);
            }
        }
        if s < n - m {
            h_t = (d * (h_t + q - (text_bytes[s] as u64 * d_m) % q) + text_bytes[s + m] as u64) % q;
        }
    }

    result
}

pub fn kmp(text: &str, pattern: &str) -> Vec<usize> {
    let text_bytes = text.as_bytes();
    let pattern_bytes = pattern.as_bytes();
    let mut result = Vec::new();

    if pattern.is_empty() || pattern.len() > text.len() {
        return result;
    }

    // Prefix function
    let mut lps = vec![0; pattern.len()];
    let mut len = 0;
    for i in 1..pattern.len() {
        while len > 0 && pattern_bytes[i] != pattern_bytes[len] {
            len = lps[len - 1];
        }
        if pattern_bytes[i] == pattern_bytes[len] {
            len += 1;
            lps[i] = len;
        }
    }

    // Search
    let mut i = 0;
    let mut j = 0;
    while i < text.len() {
        if text_bytes[i] == pattern_bytes[j] {
            i += 1;
            j += 1;
        }
        if j == pattern.len() {
            result.push(i - j);
            j = lps[j - 1];
        } else if i < text.len() && text_bytes[i] != pattern_bytes[j] {
            if j != 0 {
                j = lps[j - 1];
            } else {
                i += 1;
            }
        }
    }

    result
}

pub fn boyer_moore(text: &str, pattern: &str) -> Vec<usize> {
    let n = text.len();
    let m = pattern.len();
    let mut result = Vec::new();

    if m == 0 || m > n {
        return result;
    }

    let mut bad_char = [m; 256];
    for (i, &b) in pattern.as_bytes().iter().enumerate().take(m - 1) {
        bad_char[b as usize] = m - 1 - i;
    }

    let text_bytes = text.as_bytes();
    let pattern_bytes = pattern.as_bytes();
    let mut i = 0;

    while i <= n - m {
        let mut j = (m - 1) as isize;
        while j >= 0 && pattern_bytes[j as usize] == text_bytes[i + j as usize] {
            j -= 1;
        }
        if j < 0 {
            result.push(i);
            i += 1;
        } else {
            let shift = bad_char[text_bytes[i + j as usize] as usize];
            i += shift.max(1);
        }
    }

    result
}
