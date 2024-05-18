fn main() {
    println!("Hello, world!");

    // all the programs that we are running will put in RAM
    // the binary of our application will be stored on SSD
    // but whenever we open it need to execute, so it will execute
    // in RAM

    // In JS the memmory management will be completely handled by Garbage collector, or other like stuffs
    // In c like low-level language - we have to manually need to manage memmory

    // In rust we have different way
    // Rust has it's own """"ownership model""""" for memmory managment
    // makes it extremely safe no memmory errors

    // rust is forcing some rules to follow
    // this will make to avoid the memmory issues
    // we have the access like c in rust alsso

    // not having a garbage collector is one of the key reason for rust is so foast

    // #1 Mutability
    //========================

    let x: i32 = 1;
    // x = 100 ❌

    //by default in rust everything is immutable
    // immutable => not possible to make any modification

    //explicitly want to say that it's mutable

    let mut y: i32 = 1000;

    // one of the reason because the variables are immutable by default is
    // for example
    // if 2 thread is accessing a variable, and if the variable is mutable, if the value changed during the runtime
    // and the 2 threads accessing it, so may be there will arise a race condition
    // the other thread will also will get the unexpected change
    // sooooooooooo guyzzz, if it's immutable
    // it's thread safe, it helps the compiler to optimize the code

    // #2 Stack VS Heap
    //========================

    // stack is a organized ds where located in RAM to store the variable
    // whose datatype or the length will not change, it will located on Stack
    // eg
    let num1: i32 = 10;
    let num2: bool = 30;
    // the above ones not going to change their memmory allocated space runtime
    // so it will be stored on stack, and it's organized

    // heap is unorganized ds, it stores variable whose length, or the storage that takes
    // in memmory is dynamic, or it can only decide at runtime
    // it will use heap, and it's not an organized memmory
    // eg
    let str1 = String::from("HELLO");
    str1.push_str("NABEEEL");
    // but still some reference to the heap memmory will be stored on stack, that will
    // point to the exact memmory in heap

    // similar to js call stack
    // whenever a function call it will stack frame and push to the stack
    // inside the stack frame the memmory will be allocated for the variables



    //#3 Ownership
    //==============================

    //stack memmory does not make any issue with ownership
    //for example
    let n1: i32 = 10;
    let n2: i32 = 100;
    println!("Hello sum of {} & {} is {}", n1, n2, do_something(n1, n2));


    //ownership will comes whenever the heap memmory is created
    //for example

    let s1 = String::from("This is a string");
    let s2 = s1; // now the ownership is changed
    //because as you thing in the working of rust
    //in the stack frame there will be new memmory will created for the pointer
    // for the s2 as expected, the refrence is same to the string
    // it will not create any copy of the existing string in heap
    // so what happen now the s1 is invalid

    //it helps us to avoid such errors like
    //dangling pointers, double free errors

    //another example

    let new_str = String::from("New str for demonstration");
    // take_ownership(new_str);
    // println!("No longer the statment valid {}", new_str); throws errror ❌

    // Bbbbbbbbut
    // you could do this

    // take_clone(new_str.clone());
    // println!("It's completely valid {}", new_str);
    // but still it's expensive, we have to ask for more space in heap
    
    
    // we can also pass back the ownership
    // for example
    
    //let mut new_str = String::from("New str for demonstration"); make this mutable
    // new_str = take_ownership_and_give_back(new_str);
    // println!("Now It's completely valid {}", new_str);

    // #Borrowing and Refrences
    // ===========================================

    // The problem with the ownership will forget once we understand borrowing

    //eg
    let j1 = String::from("Hello");
    let j2 = &j1; // now the j2 borrowed j1

    println!("{}", j1);// completly valid
    println!("{}", j2);// completly valid
    
    // it means it's doesn't pointing directly to "Hello" string on heap memmory
    // instead it pointing through the j1, basically it depending on j1
    // once the use of j2 end, it will not delete the string from heap
    // because it's not the owner of the string, once the j1 excuted then the memmory will deallocated
    // because still j1 is the owner 

    // similary we can use this to pass variables to another function
    take_borrow(&j1);
    println!("{}", j1);// completly valid

    // Mutable Refrence
    // if we need to make changes in the data using borrower, then there should be
    // a single borrower, if there is multiple borrower then it throws error

    take_mutable_borrow(&mut j1);
    println!("Updated string {}", j1); // it will output the updated string - no problem
    // but the rule is there now we can't again create a mutable refrence or a borrow with it

    // let j3: &mut String = &mut j1; // it will throw errror ❌
    // le j4: &String = &j1; // it will also throw error ❌

    // WWWWWWWhat is rust
    // memmory safe language, right !!!
    // this is to avoid data races, inconsistent behaviour

    // Life times --is important next #5topic
    // String slice
}



fn take_ownership(str: String) {
    println!("Ownership taken {}", str);
}

fn take_borrow(str: &String) {
    println!("borrow taken {}", str);
}

fn take_mutable_borrow(str: &mut String) {
    println!("mutable borrow taken {}", str);
    str.push_str("It will work");
}

fn take_ownership_and_give_back(str: String) -> String {
    println!("Ownership taken {}", str);
    return str;
}

fn take_clone(str: String) {
    println!("It's not taken ownership it created a copy in heap");
}

fn do_something(a: i32, b:i32) -> i32 {
    let c = a + b;
    return c;
}