pub trait Decrypt {
    fn decrypt_string(&mut self, key: i8) -> String;
}

/// encrypts string with caesercipher
/// # args
/// takes string slice to the string you would like to decrypt
/// # returns
/// this function return a string
impl Decrypt for String {
    fn decrypt_string(&mut self, key: i8) -> String {
        let mut finalvalue = String::new();
        for i in self.chars() {
            if !i.is_ascii_alphabetic() {
                finalvalue.push(i);
            } else if i.is_ascii_lowercase() {
                let i = i as i8;
                let i = ((i - 97 - &key) % 26 + 97) as u8 as char;
                finalvalue.push(i);
            } else {
                let i = i as i8;
                let i = ((i - 65 - &key) % 26 + 65) as u8 as char;
                finalvalue.push(i);
            }
        }
        finalvalue
    }
}
