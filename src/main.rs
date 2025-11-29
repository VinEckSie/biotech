fn main() {
    let pattern_number = pattern_count("ATCCGATCCCATGCCCATG", "CC");
    println!("Number of pattern: {}",pattern_number);
}

fn pattern_count(text: &str, pattern: &str) -> usize {
    let text_bytes = text.as_bytes();
    let pattern_bytes = pattern.as_bytes();

    let text_len = text_bytes.len();
    let pattern_len = pattern_bytes.len();

    let mut count = 0;

    for i in 0..text_len - pattern_len {
        if &text_bytes[i..i+pattern_len] == pattern_bytes {
           count += 1; 
        }
    }

    count
}