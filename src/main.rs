use std::process::exit;

use modules::{
    lock_file_controller::{read_lock_file, write_lock_file},
    prompt::ask_current,
    salah_time_fetcher,
};

mod modules;

fn main() {
    let current = salah_time_fetcher::calculate_current_salah();

    let current_lock = read_lock_file();

    if current_lock.is_empty() {
        write_lock_file(&current.name());
        ask_current(&current.name());

        exit(0);
    }

    if current_lock.replace("*", "") != current.name() {
        ask_current(&current.name());

        exit(0);
    }

    if current_lock.starts_with("*") {
        exit(0);
    }

    ask_current(&current.name());
}
