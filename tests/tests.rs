use caesercipher::{decrypt::Decrypt, encrypt::encrypt_string};

#[test]

fn encryptest1() {
    let result = encrypt_string("hello", 1);
    assert_eq!(result, "ifmmp");
}
#[test]

fn decrypttest1() {
    let result = "ifmmp".to_string().decrypt_string(1);
    assert_eq!(result, "hello");
}
