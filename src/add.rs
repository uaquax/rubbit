use core::panic;
use std::{fs::{OpenOptions}, io::Write};

use magic_crypt::{new_magic_crypt, MagicCryptTrait};

pub async fn add(url: &String, pass: &String) {
    let conf_path = ".rubbit\\.config".to_string();
    let mut conf = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .create(true)
        .open(&conf_path)
        .expect("No rubbit found");

    let mcrypt = new_magic_crypt!("magickey", 256); 
    let client_password = mcrypt.encrypt_str_to_base64(format!("{} rubbit", &pass));
    
    // TODO: GET PASSWORD FROM THE SERVER
    
    let serv_password = mcrypt.encrypt_str_to_base64("testpass rubbit");

    if client_password == serv_password {
        let text = format!("[URL] {}\n", url);
        
        conf.write(text.as_bytes()).unwrap();
    }

    else {
        panic!("Access denied");
    }
}