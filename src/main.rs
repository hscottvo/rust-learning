fn first_word(input: &str) -> &str {
    let bytes = input.as_bytes();
    for (index, &character) in bytes.iter().enumerate(){
        if character == b' ' {
            return &input[..index];
        }
    }
    &input[..]
}

fn main() {
    let s = String::from("Hello world");
    
    println!("{s}");

    let out = first_word(&s);
    println!("{out}");
}
