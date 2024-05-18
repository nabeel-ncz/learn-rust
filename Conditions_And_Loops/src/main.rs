fn main() {
    println!("Hello, world!");

    let is_even = false;
    let is_odd = false;
    
    // if condition
    if is_even {
        println!("Is even");
    } else if is_odd {
        println!("Is odd");
    } else {
        println!("No odd or even");
    }

    //for loops
    println!("Loop starts here...");
    for i in 0..10 {
        print!("{}   ", i);
    }
    println!("Loop end!!");


    //iterate over data structures like
    //arrays, strings, maps

    let str = String::from("Hello Guysss");
    let res = first_word(str);
    println!("First of word  {}", res);

}

// function find first word
fn first_word(sentence: String) -> String {
    let mut ans: String = String::from("");
    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    return ans;
}