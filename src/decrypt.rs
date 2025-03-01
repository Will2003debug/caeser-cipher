/// encrypts string with caesercipher
/// # returns
/// this function return a string
pub fn decrypt_string(string: String, key: i8) -> String {
    let mut finalvalue = String::new();
    for i in string.chars() {
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
