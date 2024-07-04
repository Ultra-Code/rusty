#[derive(Debug)]
pub enum States {
    Alabama,
    Alaska,
}

pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(States),
}

impl Coin {
    fn value_in_cents(coin: Self) -> u8 {
        match coin {
            Self::Penny => 1,
            Self::Nickel => 5,
            Self::Dime => 10,
            Self::Quarter(state) => {
                println!("State is {state:?}");
                25
            }
        }
    }
}

const fn other_match() {
    let dice = 9;
    match dice {
        7 | 9 => {}
        //_ can be use if value isn't needed
        //for no action in the catch all use the unit tuple `()`
        other_values => {
            _ = other_values;
        }
    }
}

fn if_let() -> i32 {
    let val = 0;
    let coin = Coin::Quarter(States::Alabama);
    //if let is similar to matching only one variant

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
        2
    } else {
        val + 1
    }
}
#[derive(Debug)]
enum Either {
    Left(usize),
    Right(String),
}

fn either() {
    let x = Either::Right(String::from("Hello world"));

    let value = match x {
        Either::Left(n) => n,

        Either::Right(ref s) => s.len(),
    };

    println!("{x:?} {value}");
}
