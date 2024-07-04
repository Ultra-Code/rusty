#[derive(Debug)]
struct User {
    active: bool,
    email: String,
    username: String,
    sign_in_count: u8,
}

impl User {
    const fn init_user(email: String, username: String) -> Self {
        Self {
            active: true,
            email,
            username,
            sign_in_count: 1,
        }
    }

    fn update_user(self, email: String) -> Self {
        println!("previous user {self:#?}");
        dbg!(Self { email, ..self })
    }
}

struct Point(u8, u8, u8);

const fn init_point(r: u8, g: u8, b: u8) -> Point {
    Point(r, g, b)
}
