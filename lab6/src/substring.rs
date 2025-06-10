pub fn rabin_karp(text: &str, pattern: &str) -> Vec<usize> {
    const BASE: u64 = 256;
    const MOD: u64 = 1_000_000_000;

    let n = text.len();
    let m = pattern.len();
    let mut result = Vec::new();

    if pattern.is_empty() || m > n {
        return result;
    }

    let text_bytes = text.as_bytes();
    let pattern_bytes = pattern.as_bytes();

    let mut hash_pattern = 0u64;
    let mut hash_text = 0u64;
    let mut h = 1u64;

    for _ in 0..m - 1 {
        h = (h * BASE) % MOD;
    }

    for i in 0..m {
        hash_pattern = (BASE * hash_pattern + pattern_bytes[i] as u64) % MOD;
        hash_text = (BASE * hash_text + text_bytes[i] as u64) % MOD;
    }

    for i in 0..=n - m {
        if hash_pattern == hash_text {
            if &text[i..i + m] == pattern {
                result.push(i);
            }
        }

        if i < n - m {
            hash_text = (BASE * (hash_text + MOD - text_bytes[i] as u64 * h % MOD)
                + text_bytes[i + m] as u64)
                % MOD;
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

    // Префикс-функция
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

    // Поиск
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
