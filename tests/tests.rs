use caesercipher::{decrypt::{self, decrypt_string},encrypt::{self, encrypt_string}};

#[test]

fn encryptest1() {
    let result = encrypt_string("hello".to_string(), 1);
    assert_eq!(result,"ifmmp".to_string());
}
#[test]

fn decrypttest1() {
    let result = decrypt_string("ifmmp".to_string(), 1);
    assert_eq!(result,"hello");
}