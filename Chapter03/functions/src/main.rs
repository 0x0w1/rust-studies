fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let five = five();
    println!("The five() method return {five}");

    let six = plus_one(five);
    println!("The value of six() method return {six}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}