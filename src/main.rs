use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Single command line argument
    println!("{}",args[1]);
    
    // Parse command line arguments
    /* 
    for argument in args.iter() {
        println!("{}", argument);
    }
    */
}