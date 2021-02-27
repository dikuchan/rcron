use crate::*;

use std::fs::remove_file;

use common::job::Job;
use home::home_dir;

#[test]
fn test_daemon_cache() {
    let mut scheduler = Scheduler::new();

    for i in 0..10 {
        let command = format!("{}", i);
        let args = vec!["-l"; 10];
        let time = i64::MAX;
        let job = Job::new(&command, args, time).unwrap();

        scheduler.insert(i as u64, job);
    }

    let mut path = home_dir().unwrap();
    path.push("rcron-test-temp");

    let _ = scheduler.save(&path);
    let scheduler = Scheduler::load(&path).unwrap();

    for (i, job) in (0..10).zip(scheduler.iter()) {
        let (_, job) = job;
        assert_eq!(job.command, format!("{}", i).to_owned());
        assert_eq!(job.args, vec!["-l".to_owned(); 10]);
        assert_eq!(job.time, i64::MAX as u64);
    }

    let _ = remove_file(&path);
}
