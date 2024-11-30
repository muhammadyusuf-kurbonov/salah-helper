use std::fs;

pub fn write_lock_file(salah: &String) {
    fs::write("./salah.lock", salah).expect("Unable to read file")
}

pub fn read_lock_file() -> String {
    let mut lock_file = String::new();
    if fs::read_to_string("./salah.lock").is_ok() {
        lock_file.push_str(&fs::read_to_string("./salah.lock").unwrap());
    }
    lock_file
}
