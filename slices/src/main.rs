fn main() {
    let sentence = String::from("Hello, world!");
    let word = first_word(&sentence);
    println!("{word}");

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    let mut x = &mut 5;
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
