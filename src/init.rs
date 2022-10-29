use std::{fs, process::Command, path::Path, io::Write};

pub fn init() {
    let path: String = ".rubbit".to_string();

    create_folder(&path);
    create_config(&path);
}

fn create_folder(path: &String) {
    if Path::new(&path).exists() {
        fs::remove_dir_all(&path).expect("Can not remove folder");
    }
    fs::create_dir_all(&path).expect("Can not create folder");

    set_hidden(&path);

    println!("Folder created");
}

fn create_config(path: &String) {
    let conf_path = format!("{}\\{}", &path, ".config");

    if Path::new(&conf_path).exists() {
        fs::remove_file(&conf_path).expect("Can not remove config");
    }
    
    let mut conf = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(&conf_path)
        .expect("Can not create config");
    conf.write(b"[TYPE] client\n").unwrap();

    set_hidden(&conf_path);

}

fn set_hidden(path: &String) {
    Command::new("attrib")
        .arg("+h")
        .arg(path)
        .output().expect("Can not hide folder");
}