extern crate chrono;

mod constants {
    pub const FILENAME: &str = "app.conf";
    pub const FILEAPP: &str = "file";
    pub const START: &str = "start";
    pub const REPEAT: &str = "repeat";
}

struct Config {
    file_app: std::string::String,
    start: chrono::NaiveTime,
    repeat: u64
}

impl Default for Config {
    fn default () -> Config {
        Config {
            file_app: std::string::String::new(),
            start: chrono::NaiveTime::from_hms(0, 0, 0),
            repeat: 24
        }
    }
}

fn main() {
    use chrono::Timelike;

    // checking file configured
    let filename = constants::FILENAME;
    let config_lines = std::fs::read_to_string(filename)
        .expect("File app.conf not found. Please, create it on the same directory where the executable.");
    let config_lines: Vec<&str> = config_lines.split('\n').collect();
    
    let mut config = Config { ..Default::default() };
    for line in config_lines {
        let split: Vec<&str> = line.split('=').collect();
        let identifier = split[0];
        let conf_value = get_conf_value(&line);
        match identifier {
            constants::FILEAPP => config.file_app = std::string::String::from(get_str(&conf_value)),
            constants::START => config.start = get_naivetime(&conf_value),
            constants::REPEAT => config.repeat = get_u64(&conf_value),
            _ => println!("Error trying to parse {}, not expected value", &line),
        }
    }

    // calc diff between current date and date configured, and set the timer
    println!("[Setting timer]");
    let hours_to_wait: u64 = config.repeat;
    println!("Timer set succesfully every {} hours", hours_to_wait);

    // after setting the timer, now the program will be executed every x hours (programmed in the config file)
    println!("[Timer schedule]");
    let sleep_time_hours = std::time::Duration::from_secs(hours_to_wait);
    let now = chrono::Local::now();
    let mut naive_date_time = chrono::NaiveDateTime::new(now.naive_local().date(), now.naive_local().time());

    let mut first_iteration = true;
    loop {

        if first_iteration {
            let first_sleep = calc_first_sleep(config.start.hour());

            println!("Next execution: {}", 
                naive_date_time + chrono::Duration::from_std(first_sleep)
                    .unwrap());
            println!("Sleeping for {} hours", first_sleep.as_secs() / 3600);

            std::thread::sleep(first_sleep);

            naive_date_time = naive_date_time + chrono::Duration::from_std(first_sleep)
                .unwrap();
            first_iteration = !first_iteration;
        } else {
            println!("Next execution: {}", 
                naive_date_time + chrono::Duration::from_std(sleep_time_hours)
                    .unwrap());
            println!("Sleeping for {} hours", 
                sleep_time_hours.as_secs() / 3600);

            std::thread::sleep(sleep_time_hours);

            naive_date_time = naive_date_time + chrono::Duration::from_std(sleep_time_hours)
                .unwrap();
        }

        println!("Executing task at {}", naive_date_time);
        execute(&config.file_app);
    }
}

fn get_conf_value(line: &str) -> &str {
    let vec: Vec<&str> = line.split('=').collect();
    vec.get(1).expect("There is no value on line").trim()
}

fn get_str(conf_value: &str) -> &str {
    conf_value
}

fn get_naivetime(conf_value: &str) -> chrono::NaiveTime {
    chrono::NaiveTime::parse_from_str(conf_value, 
        "%H:%M").expect("Error parsing to chrono::NaiveTime")
        

}

fn get_u64(conf_value: &str) -> u64 {
    conf_value.parse::<u64>()
        .expect("Error parsing value to u64")
}

fn calc_first_sleep(hour: u32) -> std::time::Duration {
    use chrono::{Local, Timelike};

    let current_hour = Local::now().hour();

    if hour > current_hour {
        chrono::Duration::hours((hour - current_hour) as i64).to_std().unwrap()
    } else {
        chrono::Duration::hours((24 + hour - current_hour) as i64).to_std().unwrap()
    }
}

fn execute(file: &std::string::String) {
    println!("Executing {}", file);
    let output = std::process::Command::new("cmd")
        .args(&["/C", file])
        .output()
        .expect("failed to execute process");
 
 
    for out in String::from_utf8(output.stdout).iter() {
        println!("{}", out);
    }
}