#![warn(dead_code)]


fn three_vowels_string(word: &String) -> bool {
    let mut vowel_count = 0;
    for c in word.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowel_count += 1;
                if vowel_count >= 3 {
                    return true
                }
            }
            _ => vowel_count = 0
        }
    }
    false
}

fn three_vowels_str(word: &str) -> bool {
    let mut vowel_count = 0;
    for c in word.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowel_count += 1;
                if vowel_count >= 3 {
                    return true
                }
            }
            _ => vowel_count = 0
        }
    }
    false
}

#[test]
fn main() {
    let ferris = "Ferris".to_string();
    let curious = "Curious".to_string();
    // Stringを引数
    // 借用じゃないと println! 解決時にferris
    println!("{}: {}", ferris, three_vowels_string(&ferris));
    println!("{}: {}", curious, three_vowels_string(&curious));

    // &strを引数
    println!("Ferris: {}", three_vowels_str("Ferris"));
    println!("Curious: {}", three_vowels_str("Curious"));

    
    // 文字列操作の操作を行うと返り値は借用型の値が標準
    // 自作する関数の引数、返り値は借用型のほうが良い
    let sentence_string =
        "Once upon a time, there was a friendly curious crab named Ferris";
    for word in sentence_string.split(' ') {
        if three_vowels_str(word) {
            println!("{} has three consecutive vowels!", word);
        }
    }
}