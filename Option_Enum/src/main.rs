fn main() {
    println!("Hello, world!");

    // Option Enum in rust is used to handle 
    // the concept of nullability in a safe and expressive way
    // Because rust doesn't have the NULL value

    // for example i write a problem to find the first a character in a word

    let word = String::from("Hello, I am Nabeel");
    let res = find_first_a(word);
    match res {
        Some(index) => {
            println!("A character found at {}", index);
        },
        None => {
            println!("A character not found!");
        }
    }
    
}


fn find_first_a(str: String) -> Option<i32> {
    for (index, char) in str.chars().enumerate() {
        if char === "a" {
            return Some(index as i32);
        }
    }
    return None;
}