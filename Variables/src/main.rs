fn main() {
    //signed integer
    let x: i32 = 10;
    let xx: i8 = 1;
    let xxx: i64 = 10000;

    //unsigned integer -> if the number will only be a positive
    let y: u32 = 10;
    let yy: u8 = 1;
    let yyy: u64 = 100000;

    //floating numbers
    let z: f32 = 10.000;

    //all variable are immutable by default
    //mutable variable
    let mut num: i32 = 100;
    println!("num: {}", num);
    
    num = 1000;
    println!("num reassigned: {}", num);


    //boolean variable
    let is_male: bool = false;
    let is_mature: bool = true;

    //if condition
    if is_male {
        println!("Is male");
    } else {
        println!("Is female");
    }

    if is_male && is_mature {
        println!("The male is matured");
    }

    println!("Hello, world!");
    println!("x: {}, xx:{}, xxx: {}", x, xx, xxx);
    println!("y: {}, yy:{}, yyy: {}", y, yy, yyy);
    println!("z: {}", z);

    //primitive datatypes number, strings, booleans

    // strings
    //they don't have a fixed type, 
    //something similar to arrays or vectors
    //they can change their length at runtime

    let str = "nabeeell";
    let str2: String = String::from("greeeeeeeeetings.........");
    let str3: &str = "nabeeeeeeeeeel...";

    println!("{}, {}, {}", str, str2, str3);
    
    //this can't be print directly
    //because it's returning a optional character
    let char1fromstr: Option<char> = str.chars().nth(1);
    //pr  println!("{}", char1fromstr); ❌
    
    //because rust doesn't know wether it's a character or undefined
    //rust want to know it's a character then only it's going to print
    
    
    // Some -> will work when something exist in char1fromstr variable
    // None -> will work when nothing is there
    // match char1fromstr {
    //     Some(ch: char) => println!("{}", ch),
    //     None => println!("No character at index 1"),
    // }   

    //or you could use
    println!("{}", char1fromstr.unwrap()); 
}
