use std::collections::HashMap;
fn is_vowel(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}
fn main() {
    let mut a = "hello".to_string();
    a.push_str("hello_2");

    let b = String::from("hi_3");

    println!("Hello, {}", a);

    let c = a + &b;
    println!("{}", c);

    let mut v = vec![1, 2, 3];

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 10;
    }

    for i in &v {
        println!("{}", i);
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    let pigify = String::from("hi there, my apple is crazy");
    for i in pigify.split_whitespace() {
        let mut string_i = String::from(i);
        let first_char = string_i.remove(0);
        let mut ret: String = String::from("");
        if is_vowel(first_char) {
            ret = String::from(i);
            ret.push_str("-hay");
        } else {
            ret = String::from(string_i);
            ret.push_str("-");
            ret.push(first_char);
            ret.push_str("ay");
        }
        println!("{0} turns into {1}", i, ret);
    }
}
