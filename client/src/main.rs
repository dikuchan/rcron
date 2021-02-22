#![feature(unix_socket_ancillary_data)]

#[macro_use]
extern crate clap;

pub mod error;

use crate::error::ClientResult;

use std::{
    os::unix::net::UnixStream,
};

use chrono::{
    offset::{Offset, TimeZone}, 
    Local, NaiveDateTime,
};
use clap::Values;
use common::{
    get_socket_path,
    plan::ExecutionPlan,
};

fn parse() -> ClientResult<ExecutionPlan> {
    let matches = clap_app!(rcron => 
        (version: "1.0")
        (author: "Dmitry K. <dikuchan@yahoo.com>")
        (@arg COMMAND: -c --command +takes_value +required "A command to launch")
        (@arg ARGS: -a --args +takes_value "Arguments of a script")
        (@arg TIME: -t --time +takes_value +required "Time to start an execution (YYYY.MM.DD HH:MM:SS)")
    ).get_matches();

    // Fails only on the `clap` level.
    let command = matches.value_of("COMMAND").unwrap();
    let args = matches.values_of("ARGS").unwrap_or(Values::default()).collect();
    let time = matches.value_of("TIME").unwrap();

    // Convert local time to UTC.
    let time = NaiveDateTime::parse_from_str(time, "%Y.%m.%d %H:%M:%S")?;
    let offset = Local.timestamp(0, 0)
        .offset()
        .fix()
        .utc_minus_local();
    let timestamp = time.timestamp() + offset as i64;

    Ok(ExecutionPlan::new(command, args, timestamp)?)
}

fn send(plan: ExecutionPlan) -> ClientResult<()> {
    let socket_path = get_socket_path();
    let socket = UnixStream::connect(socket_path)?;

    socket.set_passcred(true)?;

    Ok(plan.send(socket)?)
}

fn main() {
    match parse()
        .map_err(|e| e)
        .and_then(send)
        .map_err(|e| e) {
        Ok(_) => println!("Scheduled: ok"),
        Err(e) => println!("Cannot schedule: {}", e)
    }
}
