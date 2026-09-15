fn main() {
    let s1 = String::from("hello s1");
    let length = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, length);

    // s2가 mutable 하지 않아서 에러
    // let s2 = String::from("hello s2");

    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("{}", s2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// mutable한(가변) 참조자라고 표기해줘야 함
// fn change(some_string: &String) {
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}