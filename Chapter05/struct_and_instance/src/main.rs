struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("0x0w1"),
        email: String::from("0x0w1@gmail.com"),
        sign_in_count: 1,
    };

    println!("user1.email: {}", user1.email);
    println!("user1.active: {}", user1.active);
    println!("user1.username: {}", user1.username);

    user1.email = String::from("changed-0x0w1@gmail.com");
    println!("[=] After changed : user1.email: {}", user1.email);

    let mut user2 = build_user("TEST2User@gmail.com".to_string(), "TEST2User".to_string());
    println!("user2.email: {}", user2.email);
    println!("user2.username: {}", user2.username);

    let mut user3 = User {
        email: String::from("another@gmail.com"),
        ..user2
    };
    println!("user3.email: {}", user3.email);
    println!("user3.username: {}", user3.username);

    // 명명도니 필드 없는 튜플 구조체
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Point와 Color에서 각각 doesn't implement `Debug`
    // println!("origin: {:?}, black: {:?}", origin, black);

    let mut user4 = build_user_str("test@gmail.com", "iAmABoy");
    println!("user4.email: {}", user4.email);
    println!("user4.username: {}", user4.username);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn build_user_str(email: &str, username: &str) -> User {
    User {
        active: true,
        username: username.to_string(),
        email: email.to_string(),
        sign_in_count: 1,
    }
}
