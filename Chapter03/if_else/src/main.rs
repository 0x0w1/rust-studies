fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;

    // let number = if condition { 5 } else { "six" };
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // loop {
    //     println!("again!");
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {}", count);

    while_loop();
    for_loop();
}

fn while_loop(){
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loop(){
    // example for but using while
    let a = [ 1,2,3,4,5];
    let mut index = 0;

    while index < 5 {
        println!("[FOR-WHILE] the value is: {}", a[index]);
        index += 1;
    }

    // example real for
    for element in a {
        println!("[FOR] the value is: {}", element);
    }

    // example real for 2
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
