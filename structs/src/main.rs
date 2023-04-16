#[allow(dead_code)]
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u32,
}

impl User {
    fn login(&mut self) {
        self.active = true;
        self.sign_in_count += 1;
    }
}

impl User {
    fn new(username: &str, email: &str) -> User {
        User {
            username: String::from(username),
            email: String::from(email),
            active: false,
            sign_in_count: 0,
        }
    }
}

fn main() {
    let mut user1 = User {
        username: String::from("dursun koc"),
        email: String::from("dursunkoc@gmail.com"),
        active: true,
        sign_in_count: 1,
    };

    println!("user1 : {:?}", user1);
    user1.login();
    println!("user1 after login : {:?}", user1);

    let mut user2 = User::new("mustafa koc", "mustafakoc@gmail.com");
    println!("user2: {:#?}", user2);
    user2.login();
    println!("user2 after login: {:?}", user2);
}
