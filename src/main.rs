use caesercipher::{decrypt::decrypt_string, encrypt::encrypt_string};
use std::{
    error::Error,
    io::{self, ErrorKind},
};
fn main() -> Result<(), Box<dyn Error>> {
    let mut encrypt_or_decrypt = String::new();
    println!("would you like to encrypt or decrypt");
    io::stdin().read_line(&mut encrypt_or_decrypt)?;
    let encrypt_or_decrypt = encrypt_or_decrypt.to_ascii_lowercase();

    let mut key = String::new();
    let mut string = String::new();
    println!("enter the sentence you would like to encrypt");
    io::stdin().read_line(&mut string)?;
    println!("please enter the key you would like to encrypt with ");
    io::stdin().read_line(&mut key)?;
    let key = key.trim().parse::<i8>()?;
    let altered = match encrypt_or_decrypt.trim() as &str {
        "encrypt" => encrypt_string(string, key),
        "decrypt" => decrypt_string(string, key),
        _ => {
            return Err(Box::new(io::Error::new(
                ErrorKind::InvalidInput,
                "only encrypt or decrypt are accepted inputs",
            )));
        }
    };
    println!("{altered}");
    Ok(())
}
