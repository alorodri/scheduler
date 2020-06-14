extern crate chrono;
fn main() {

    // checking file configured
    let filename = "app.conf";
    let config_lines = std::fs::read_to_string(filename).unwrap();
    let config_lines: Vec<&str> = config_lines.split('\n').collect();
    
    let file_to_execute_from_parsed_config: &str = config_lines.get(0).unwrap();
    let date_to_start_executing_from_parsed_config: &str = config_lines.get(1).unwrap();
    let hours_to_wait_from_parsed_config: &str = config_lines.get(2).unwrap();
    println!("[Parsing values from app.config]");
    println!("{}", file_to_execute_from_parsed_config);
    println!("{}", date_to_start_executing_from_parsed_config);
    println!("{} hours", hours_to_wait_from_parsed_config);

    // checking date configured
    println!("[Parsing date configured]");
    let vector: Vec<&str> = date_to_start_executing_from_parsed_config.split('=').collect();
    let date_to_start: &str = vector.get(1).unwrap().trim();
    println!("Parsing {} to time", date_to_start);

    // calc diff between current date and date configured, and set the timer
    println!("[Setting timer]");
    let vector: Vec<&str> = hours_to_wait_from_parsed_config.split('=').collect();
    let hours_to_wait: i64 = vector.get(1).unwrap().parse::<i64>().unwrap();
    println!("Timer set succesfully every {} hours", hours_to_wait);

    // after setting the timer, now the program will be executed every x hours (programmed in the config file)
    println!("[Timer schedule]");

    let sleep_time_hours = chrono::Duration::hours(hours_to_wait);
    let now = chrono::Local::now().naive_local().date();
    let mut naive_date_time = chrono::NaiveDateTime::new(now, 
        chrono::NaiveTime::parse_from_str(date_to_start, "%H:%M")
        .unwrap());

    let file_to_execute: Vec<&str> = file_to_execute_from_parsed_config.split('=').collect();
    let file_to_execute: &str = file_to_execute.get(1).unwrap();

    let mut first_iteration = true;
    loop {
        println!("Next execution: {}", naive_date_time + sleep_time_hours);
        naive_date_time = naive_date_time + sleep_time_hours;
        if first_iteration {
            std::thread::sleep(calc_first_sleep(naive_date_time)
                .to_std().
                unwrap());
            first_iteration = !first_iteration;
        } else {
            std::thread::sleep(sleep_time_hours.to_std()
                .unwrap());
        }
        println!("Executing task at {}", naive_date_time);
        execute(file_to_execute);
    }
}

fn calc_first_sleep(mut ndt: chrono::NaiveDateTime) -> chrono::Duration {
    let local_now = chrono::Local::now();
    let local_now = chrono::NaiveDateTime::new(local_now.naive_local().date(),
    local_now.naive_local().time());
    if ndt < local_now {
        ndt += chrono::Duration::days(1);
    }
    ndt - local_now
}

fn execute(file: &str) {
    println!("Executing {}", file);
    let output = std::process::Command::new("cmd")
        .args(&["/C", file])
        .output()
        .expect("failed to execute process");
 
 
    for out in String::from_utf8(output.stdout).iter() {
        println!("{}", out);
    }
}