struct User {
    active: bool,
    name: String,
    email: String,
    sign_in_count: u32,
}

fn main() {
    println!("Hello, world!");

    // struct in rust used structure the data together
    // in rust we can't directly create an object, so that
    // first we need to create struct for that
    // similar type in typescript

    let user1 = User {
        active: true,
        name: String::from("hello"),
        email: String::from("hello@email.com"),
        sign_in_count: 10
    };

    println!("{}", user1.name);

    //there is also other types of structs

    // tuple struct, unit struct


    // you can also implement in structs
    // which means you can attach functions to instance of structs
    // very similar to callasses in ts

    
}
struct Rect {
    width: u32;
    height: u32;
}

impl Rect {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
}

