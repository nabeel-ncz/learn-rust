fn main() {
    println!("Hello, world!");

    let num1 = 10;
    let num2 = 40;
    let sum = do_sum(num1, num2);

    println!("Sum of {} and {} is {}", num1, num2, sum);

}

//return type of functions can't be infered
//so we want to manually give it
fn do_sum(a: i32, b: i32) -> i32 {
    let ans: i32;
    ans = a + b;
    return ans;
}
