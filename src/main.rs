use std::{
    fs::{create_dir, File},
    io::{Read, Write},
    path::PathBuf,
    process::{self, Command},
    thread,
    time::Duration,
};

use dirs::config_dir;
use job_scheduler::{Job, JobScheduler};
use notify_rust::Notification;
use serde::{Deserialize, Serialize};
use system_shutdown::shutdown;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    poll_frequency: u32,
    threshold: i32,
}

fn main() {
    println!("started safe shutdown");

    let config_name: &str = "safe_shutdown/config.yaml";

    thread::sleep(Duration::from_millis(1000));

    println!("retrieving config file");

    let config_path = {
        let o = config_dir().unwrap();
        let cnpn = o.as_path().join(config_name);
        cnpn
    };

    match config_path.exists() {
        true => listen(config_path),
        false => create_defualt_config(config_path),
    }
}

fn create_defualt_config(p: PathBuf) {
    println!("cound not find config file. creating default one");

    let org = p.clone();
    // create the config then pass it to listen fn
    let config: Config = toml::from_str(
        r#"        
        poll_frequency = 10  
        threshold = 10   
    "#,
    )
    .unwrap();

    assert_eq!(config.poll_frequency, 10);
    assert_eq!(config.threshold, 10);

    println!("{:?}", p);

    let dir = p.parent().unwrap();
    create_dir(dir).expect("failed to create config directory");

    println!("stored at {:?}", p.to_str());

    let mut file = File::create(p.to_str().unwrap()).expect("error while creating config file");

    let tml = toml::to_string(&config).unwrap();

    file.write(tml.as_bytes())
        .expect("could not write to config toml file");

    listen(org);
}

fn listen(p: PathBuf) {
    println!("config file found at {:?}. setting up client", p);

    let mut file = File::open(p.to_str().unwrap()).expect("failed to open config file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("failed to read config file");

    let config: Config = toml::from_str(&contents).unwrap();

    if config.poll_frequency < 1 || config.poll_frequency > 59 {
        println!(
            "Invalid poll_frequency. Expected to range 1 - 59. Found {}",
            config.poll_frequency
        );
        process::exit(0);
    }

    let mut sched = JobScheduler::new();

    let tick = format!("1/{} * * * * *", config.poll_frequency).parse();

    sched.add(Job::new(tick.unwrap(), || {
        println!("checking for battery");

        let resp = Command::new("cat")
            .arg("/sys/class/power_supply/BAT0/capacity")
            .output();

        match resp {
            Ok(r) => {
                let out = r.stdout;
                let out = String::from_utf8(out).unwrap();
                let out = out.trim();

                println!("battery level: {:?}", out);

                let level = out.parse::<i32>().unwrap();

                if level <= config.threshold {
                    Notification::new()
                        .summary("Safe Shutdown")
                        .body("Time to Shutdown")
                        .show()
                        .unwrap();

                    match shutdown() {
                        Ok(_) => println!("Shutting down, bye!"),
                        Err(error) => eprintln!("Failed to shut down: {}", error),
                    }
                }
            }
            Err(_err) => {
                Notification::new()
                    .summary("Safe Shutdown")
                    .body("Failed to get battery capacity")
                    .show()
                    .unwrap();
            }
        }
    }));

    loop {
        sched.tick();
        std::thread::sleep(Duration::from_millis(1500));
    }
}
