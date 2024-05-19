fn main() {
    println!("Hello, world!");

    // Every language will have compilation and run-time error
    // we look into how to handle run-time error

    // In js or similar languages we use try-catch block to handle the errors
    // same thing - in rust we use Result enum to gracefully handle the errors

    let res = fs::read_to_string("example.txt");
    // this is actually return Result<String, Error> this kind of type
    // so what we could do, yaaa.... pattern matching

    match res {
        Ok(content) => {
            println!("File read success {}", content);
        },
        Err(error) => {
            println!("File read error {}", error);
        }
    }

    // this is how we will handle error with breaking the code
    
}
