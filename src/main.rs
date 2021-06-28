use std::{env, thread, time};
use std::process::Command;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let task_num = &arguments[1];
    let time: u32 = arguments[2].parse().unwrap();
    // start the task
    let task_start_output = Command::new("task")
        .arg("start")
        .arg(task_num)
        .output()
        .expect("Failed to start the task");
    println!("{}", String::from_utf8(task_start_output.stdout).unwrap());
    let time_in_seconds = time * 60;
    // will want to output every second
    let one_second = time::Duration::new(1,0);
    // loop time in seconds times
    for _x in 0..time_in_seconds { 
        thread::sleep(one_second);
        let time_output = Command::new("timew")
            .output()
            .expect("Failed to check time");
        let time_string: String = String::from_utf8(time_output.stdout).unwrap();
        let time = time_string.split(" ").last();
        print!("CURRENT TIME: {} \r", time.unwrap().strip_suffix('\n').unwrap());
    }
    // stop the task after we have looped for correct amount of time
    let task_stop_output = Command::new("task")
        .arg("stop")
        .output()
        .expect("Failed to stop the task");
    println!("{:?}", task_stop_output);


}

/*
fn get_time() { 
}

fn end_session() { 
}
*/
