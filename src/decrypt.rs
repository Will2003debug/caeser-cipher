pub trait Decrypt {
    fn decrypt_string(&mut self, key: i8) -> String;
}

impl Decrypt for String {
    /// encrypts string with caesercipher
    /// # args
    /// takes string slice to the string you would like to decrypt
    /// # returns
    /// this function return a string
    /// # example
    /// ```
    /// use caesercipher::decrypt::Decrypt;
    /// let result = "a".to_string().decrypt_string(1);
    /// assert_eq!(result, "z");
    /// ```
    #[allow(clippy::perf)]
    fn decrypt_string(&mut self, key: i8) -> String {
        let mut finalvalue = String::new();
        self.chars().for_each(|i| {
            if !i.is_ascii_alphabetic() {
                finalvalue.push(i);
            } else if i.is_ascii_lowercase() {
                let i = i as i8;
                let i = ((i - 97 - key + 26) % 26 + 97) as u8 as char;
                finalvalue.push(i);
            } else {
                let i = i as i8;
                let i = ((i - 65 - key + 26) % 26 + 65) as u8 as char;
                finalvalue.push(i);
            }
        });
        finalvalue
    }
}
