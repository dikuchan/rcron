pub mod error;
pub mod plan;

use std::{
    fs::create_dir_all,
    path::PathBuf,
};

use home::home_dir;

pub static RCRON_PATH: &'static str = "/run/rcron";
pub static SOCKET_NAME: &'static str = "rcron-socket";

fn get_rcron_path() -> PathBuf {
    match home_dir() {
        Some(mut path) => {
            path.push(".rcron");
            path
        },
        None => PathBuf::from(RCRON_PATH)
    }
}

pub fn get_socket_path() -> PathBuf {
    let mut path = get_rcron_path();
    path.push(SOCKET_NAME);
    path.set_extension("sock");

    path
}

pub fn get_scheduler_path() -> PathBuf {
    let mut path = get_rcron_path();
    path.push("scheduler");
    path.set_extension("bin");

    path
}

pub fn get_journal_path() -> PathBuf {
    let mut path = get_rcron_path();
    path.push("journal");
    path.set_extension("log");

    path
}

pub fn create_rcron_directory() -> std::io::Result<()> {
    create_dir_all(get_rcron_path())?;

    Ok(())
}

#[link(name = "c")]
extern "C" {
    pub fn geteuid() -> u32;

    pub fn getegid() -> u32;
}
