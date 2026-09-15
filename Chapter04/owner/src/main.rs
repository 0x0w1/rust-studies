fn main() {
    // let s = String::from("hello");
    // println!("{}", s);

    // error[E0596]: cannot borrow `s` as mutable, as it is not declared as mutable
    // let s = String::from("hello");
    // s.push_str(", world!");

    // warning: variable does not need to be mutable
    // let mut s = String::from("hello");
    // println!("{}", s);

    // let mut s = String::from("hello");
    // s.push_str(", world!");
    // println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1;

    // error[E0382]: borrow of moved value `s`
    // println!("{}",s1);
    println!("{}",s2);

    let s3 = String::from("hello");
    let s4 = s3.clone();

    println!("s3: {}, s4: {}",s3, s4);
    println!("s3 pointer: {:p}, s4 pointer: {:p}",s3.as_ptr(), s4.as_ptr());

    ownership_test();
}

fn ownership_test() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);  // let s3 = s2;

    // error[E0382]: borrow of moved value: `s2`
    // println!("s1: {}, s3: {}, s3: {}",s1, s2, s3);
    println!("s1: {}, s3: {}",s1, s3);
}

fn gives_ownership() -> String {
    let yours = String::from("yours");
    yours
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}