use std::{fs, process::Command, path::Path, io::Write};
use magic_crypt::{MagicCryptTrait, new_magic_crypt};

pub fn initserv(pass: &String) {
    let path: String = ".rubbit".to_string();

    create_folder(&path);
    create_config(&path,  &pass);
}

fn create_folder(path: &String) {
    if Path::new(&path).exists() {
        fs::remove_dir_all(&path).expect("Can not remove folder");
    }
    fs::create_dir_all(&path).expect("Can not create folder");

    set_hidden(&path);

    println!("Folder created");
}

fn create_config(path: &String, pass: &String) {
    let conf_path = format!("{}\\{}", &path, ".config");

    if Path::new(&conf_path).exists() {
        fs::remove_file(&conf_path).expect("Can not remove config");
    }
    
    let mut conf = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&conf_path)
        .expect("Can not create config");
    
    set_hidden(&conf_path);

    let mcrypt = new_magic_crypt!("magickey", 256); 
    let encrypted_string = mcrypt.encrypt_str_to_base64(format!("{} rubbit", &pass));
    let text = format!("[PASSWORD] {}\n", encrypted_string);

    conf.write(b"[TYPE] server\n").unwrap();
    conf.write(text.as_bytes()).unwrap();

    println!("Config created");
}

fn set_hidden(path: &String) {
    Command::new("attrib")
        .arg("+h")
        .arg(path)
        .output().expect("Can not hide folder");
}