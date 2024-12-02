use std::fs;
use home::{self, home_dir};

pub fn write_lock_file(salah: &String) {
    fs::write(home_dir().expect("Could not find home dir path").as_path().join("salah.lock"), salah).expect("Unable to read file")
}

pub fn read_lock_file() -> String {
    let mut lock_file = String::new();
    if fs::read_to_string(home_dir().expect("Could not find home dir path").as_path().join("salah.lock")).is_ok() {
        lock_file.push_str(&fs::read_to_string(home_dir().expect("Could not find home dir path").as_path().join("salah.lock")).unwrap());
    }
    lock_file
}
