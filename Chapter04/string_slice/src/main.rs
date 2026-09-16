fn main() {
    // let mut s = String::from("hello");
    // let word = first_word(&s);
    // s.clear();
    // println!("The first word is: {}", word);

    let my_string = String::from("hello world");

    let word = first_word(&my_string[0..6]);
    println!("1. The word is: {}", word);
    let word = first_word(&my_string[..]);
    println!("2. The word is: {}", word);

    let word = first_word(&my_string);
    println!("3. The word is: {}", word);

    let my_string_literal = "hello world";

    let word = first_word(&my_string_literal[0..6]);
    println!("4. The my_string_literal is: {}", word);
    let word = first_word(&my_string_literal[..]);
    println!("5. The my_string_literal is: {}", word);
    let word = first_word(my_string_literal);
    println!("6. The my_string_literal is: {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
