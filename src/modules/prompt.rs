use dialoguer::Select;

use super::lock_file_controller::write_lock_file;

pub fn ask_current(salah: &String) -> bool {
    let answer = Select::new()
        .with_prompt(format!("Do you pray salah {salah}?"))
        .items(&["Yes", "No"])
        .default(0)
        .interact()
        .unwrap();

    let done = if answer == 0 { true } else { false };

    if done {
        write_lock_file(&format!("*{salah}"));
    }
    return done;
}
