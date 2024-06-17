#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]

mod guessing_game;

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

fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let mut dst_cpy = dst.clone();
    let largest = dst.iter().max_by_key(|s| s.len()).unwrap().len();

    src.iter()
        .filter(|str| str.len() > largest)
        .cloned()
        .for_each(|str| dst.push(str));

    for s in src {
        if s.len() > largest {
            dst_cpy.push(s.clone());
        }
    }

    assert_eq!(&dst_cpy, dst);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    let mut index = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            index = i;
            break;
        }
    }
    if index == 0 {
        index = s.len();
    }

    let space_index = bytes
        .iter()
        .enumerate()
        // .filter(|(_index, &byte)| byte == b' ')
        // .take(1)
        // .next()
        .find(|(_index, &byte)| byte == b' ')
        .unwrap_or((s.len(), &(0x0_u8)))
        .0;

    assert_eq!(space_index, index);
    &s[..space_index]
}

#[allow(dead_code)]
fn return_a_string(output: &mut String) {
    output.replace_range(.., "Hello world");
}

fn main() {
    let mut dst = vec![String::from("The"), String::from("word")];
    let src = [String::from("Longest"), String::from("World")];
    add_big_strings(&mut dst, &src);
    first_word("Hello World");
    let y = {
        let x = 20;
        x + 3
    };

    let condition = true;
    let x = if condition { 5 } else { 6 };

    let mut counter = 0;
    let val = 'blk: loop {
        counter += 1;

        if counter == 3 {
            break 'blk counter * 3;
        }
    };
    println!("The value of x+y+val is: {}", x + y + val);

    for element in (1..=4).rev() {
        println!("{element}");
    }
    let boxed = Box::new([0; 1_000]);
    let _ = boxed;
}
