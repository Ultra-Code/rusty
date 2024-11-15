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
        .find(|&(ref _index, &byte)| byte == b' ')
        .unwrap_or((s.len(), &(0x0_u8)))
        .0;

    assert_eq!(space_index, index);
    &s[..space_index]
}

#[allow(dead_code)]
fn return_a_string(output: &mut String) {
    output.replace_range(.., "Hello world");
}

pub fn use_algos() {
    let mut dst = vec![String::from("The"), String::from("word")];
    let src = [String::from("Longest"), String::from("World")];
    add_big_strings(&mut dst, &src);
    first_word("Hello World");
}
