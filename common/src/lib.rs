pub mod error;
pub mod job;

use std::{
    fs::create_dir_all,
    path::PathBuf,
};

use home::home_dir;

pub static CACHE_PATH: &'_ str = "/var/cache";
pub static JOURNAL_PATH: &'_ str = "/var/log";
pub static SOCKET_PATH: &'_ str = "/run/rcron";
pub static SOCKET_NAME: &'_ str = "rcron-socket";

/// Tries to use a directory at `$HOME`.
/// If fails, uses `/run`.
fn get_socket_dir() -> PathBuf {
    match home_dir() {
        Some(mut path) => {
            path.push(".rcron");
            path
        },
        None => PathBuf::from(SOCKET_PATH)
    }
}

pub fn create_socket_dir() -> std::io::Result<()> {
    create_dir_all(get_socket_dir())?;

    Ok(())
}

pub fn get_socket_name() -> PathBuf {
    let mut path = get_socket_dir();
    path.push(SOCKET_NAME);
    path.set_extension("sock");

    path
}

/// Tries to use a directory at `$HOME`.
/// If fails, uses `/var`.
pub fn get_cache_name() -> PathBuf {
    let mut path = match home_dir() {
        Some(mut path) => {
            path.push(".cache");
            path
        },
        None => PathBuf::from(CACHE_PATH)
    };
    path.push("scheduler");
    path.set_extension("bin");

    path
}

pub fn get_journal_name() -> PathBuf {
    let mut path = PathBuf::from(JOURNAL_PATH);
    path.push("journal");

    path
}

#[link(name = "c")]
extern "C" {
    pub fn geteuid() -> u32;

    pub fn getegid() -> u32;
}
