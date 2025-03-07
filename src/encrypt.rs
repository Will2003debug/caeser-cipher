pub trait Encrypt {
    fn encrypt_string(&mut self, key: i8) -> String;
}

impl Encrypt for String {
        
    /// this function encrypts a string with caesercipher
    /// # args 
    /// takes string slice to the string you would like to encrypt
    /// # returns
    /// This function return a String
    /// # example
    /// ```
    /// use caesercipher::encrypt::Encrypt;
    /// let mut toencrypt = String::from("cats are the best");
    /// let result = toencrypt.encrypt_string(1);
    /// assert_eq!(result,"dbut bsf uif cftu");
    ///  ```  
    
    
    fn encrypt_string(&mut self, key: i8) -> String {
        let mut finalvalue = String::new();
        for i in self.chars() {
            if !i.is_ascii_alphabetic() {
                finalvalue.push(i);
            } else if i.is_ascii_lowercase() {
                let i = i as i8;
                let i = ((i - 97 + key) % 26 + 97) as u8 as char;
                finalvalue.push(i);
            } else {
                let i = i as i8;
                let i = ((i - 65 + key) % 26 + 65) as u8 as char;
                finalvalue.push(i);
            }
        }
        finalvalue
    }
}

