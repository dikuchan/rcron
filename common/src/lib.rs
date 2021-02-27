pub mod error;
pub mod job;

use std::{
    fs::create_dir_all,
    path::PathBuf,
};

use home::home_dir;

pub static CACHE_PATH: &'_ str = "/var/cache";
pub static LOG_PATH: &'_ str = "/var/log";
pub static SOCKET_PATH: &'_ str = "/run/rcron";
pub static SOCKET_NAME: &'_ str = "rcron-socket";

fn get_socket_dir() -> PathBuf {
    match home_dir() {
        Some(mut path) => {
            if unsafe { geteuid() } == 0 { 
                path = PathBuf::from(SOCKET_PATH);
                return path;
            }
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
            if unsafe { geteuid() } == 0 { 
                PathBuf::from(CACHE_PATH)
            } else {
                path.push(".cache");
                path
            }
        },
        None => PathBuf::from(CACHE_PATH)
    };
    path.push("rcron");
    path.set_extension("bin");

    path
}

pub fn get_journal_name() -> PathBuf {
    let mut path = get_socket_dir();
    path.push("rcron");
    path.set_extension("log");

    path
}

#[link(name = "c")]
extern "C" {
    pub fn geteuid() -> u32;

    pub fn getegid() -> u32;
}
