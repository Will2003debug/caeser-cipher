use caesercipher::{decrypt::Decrypt, encrypt::Encrypt};

#[test]

fn encryptest1() {
    let result = "hello".to_string().encrypt_string(1);
    assert_eq!(result, "ifmmp");
}
#[test]

fn decrypttest1() {
    let result = "ifmmp".to_string().decrypt_string(1);
    assert_eq!(result, "hello");
}
#[test]
fn emptytest() {
    let result = "".to_string().decrypt_string(1);
    assert_eq!(result, "");
    let result = "".to_string().encrypt_string(1);
    assert_eq!(result, "");
}
#[test]
fn symboltest() {
    let result = "/".to_string().encrypt_string(1);
    assert_eq!(result, "/".to_string());
}
