pub mod error;
pub mod job;

use std::{
    fs::create_dir_all,
    path::PathBuf,
};

pub static SOCKET_PATH: &'_ str = "/var/run";
pub static CACHE_PATH: &'_ str = "/var/cache";
pub static LOG_PATH: &'_ str = "/var/log";
pub static SOCKET_NAME: &'_ str = "rcron-socket";

fn get_socket_dir() -> PathBuf {
    let mut path = PathBuf::from(SOCKET_PATH);
    path.push("rcron");

    path
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

pub fn get_cache_name() -> PathBuf {
    let mut path = PathBuf::from(CACHE_PATH);
    path.push("rcron");
    path.set_extension("bin");

    path
}

pub fn get_journal_name() -> PathBuf {
    let mut path = PathBuf::from(LOG_PATH);
    path.push("rcron");
    path.set_extension("log");

    path
}

// `man getuid`: These functions are always successful.
pub fn get_uid() -> u32 {
    unsafe { geteuid() }
}

pub fn get_gid() -> u32 {
    unsafe { getegid() }
}

#[link(name = "c")]
extern "C" {
    pub fn geteuid() -> u32;

    pub fn getegid() -> u32;
}
