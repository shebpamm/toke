use std::fmt;
use std::error;
use std::collections::HashMap;

use secret_service::SecretService;
use secret_service::EncryptionType;

#[derive(Debug, Clone)]
pub struct CredentialDecodeError;

#[derive(Debug, Clone)]
pub struct ItemNotFoundError;

impl error::Error for CredentialDecodeError {}
impl error::Error for ItemNotFoundError {}

impl fmt::Display for CredentialDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Username not found")
    }
}

impl fmt::Display for ItemNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unable to find item from secret service")
    }
}

pub fn get_secret<'a>(ss: &'a SecretService, attributes: Vec<(&str, &str)>) -> Result<secret_service::Item<'a>, Box<dyn error::Error>> {
    return ss
        .search_items( attributes )?
        .pop()
        .ok_or(ItemNotFoundError.into())
}

pub fn get_secret_string<'a>(ss: &'a SecretService, attributes: Vec<(&str, &str)>) -> Result<String, Box<dyn error::Error>> {
    let secret = get_secret(&ss, attributes)?.get_secret()?;

    match String::from_utf8(secret) {
        Ok(result) => return Ok(result),
        Err(_error) => Err(Box::new(CredentialDecodeError)),
    }
}

pub fn set_secret<'a>(ss: &'a SecretService, label: &str, attributes: HashMap<&str, &str>, secret: &str) {
    let collection = ss.get_default_collection().unwrap();

    collection.create_item(
        label,
        attributes,
        secret.as_bytes(),
        true, // replace or not
        "text/plain"
    ).unwrap();
}


pub fn credentials_present() -> bool {
    let ss = SecretService::new(EncryptionType::Dh).unwrap();

    let username = get_secret_string(&ss, vec![("app", "toke"), ("role", "vault-username")]);
    let password = get_secret_string(&ss, vec![("app", "toke"), ("role", "vault-password")]);

    match username.and(password) {
        Ok(_value) => return true,
        Err(_error) => return false
    }
}

pub fn set_login_credentials(username: &str, password: &str) {
    let ss = SecretService::new(EncryptionType::Dh).unwrap();

    let mut username_attrs = HashMap::new();
    let mut password_attrs = HashMap::new();

    username_attrs.insert("app", "toke");
    password_attrs.insert("app", "toke");

    username_attrs.insert("role", "vault-username");
    set_secret(&ss, "toke-username", username_attrs, username);

    password_attrs.insert("role", "vault-password");
    set_secret(&ss, "toke-password", password_attrs, password);

}

pub fn get_login_credentials() -> Result<(String, String), Box<dyn error::Error>> {
    let ss = SecretService::new(EncryptionType::Dh).unwrap();

    let username = get_secret_string(&ss, vec![("app", "toke"), ("role", "vault-username")])?;
    let password = get_secret_string(&ss, vec![("app", "toke"), ("role", "vault-password")])?;

    return Ok((username, password));
}
