fn blocks() {
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

pub fn use_block() {
    blocks();
}
