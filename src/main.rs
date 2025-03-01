use caesercipher::deen;
use std::io;
fn main() {
    let mut  key = String::new();
    let mut  string = String::new();
    println!("enter the sentence you would like to encrypt");
    io::stdin().read_line(&mut string).unwrap();
    println!("please enter the key you would like to encrypt with ");
    io::stdin().read_line(&mut key).unwrap();
    let key = key.parse::<i8>().unwrap();
    let altered = deen::encrypt(string,key );
    println!("{altered}");
    
}

