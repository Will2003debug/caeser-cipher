use caesercipher::{decrypt::decrypt_string, encrypt::encrypt_string};

#[test]

fn encryptest1() {
    let result = encrypt_string("hello", 1);
    assert_eq!(result, "ifmmp");
}
#[test]

fn decrypttest1() {
    let result = decrypt_string("ifmmp", 1);
    assert_eq!(result, "hello");
}
