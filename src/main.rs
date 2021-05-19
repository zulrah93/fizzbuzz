use std::env::args;
// ./fizzbuzz 42069
fn main() {
    let result : Vec<String> = (1..args().nth(1).unwrap_or(String::from("0")).parse::<usize>().unwrap()).map(|i| match i {
        x if x % 3 == 0 && x % 5 == 0 => String::from("Fizzbuzz"),
        y if y % 5 == 0 => String::from("Buzz"),
        z if z % 3 == 0 => String::from("Fizz"),
        _ => format!("{}", i)
     } ).collect();
     
     for x in result {
         println!("{}", x);
     }
}
