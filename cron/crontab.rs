use clap::Parser;
use std::error::Error;
use std::fs;
use std::io::Read;

#[derive(Parser)]
struct CronArgs {
    #[arg(short, long)]
    edit: bool,
    #[arg(short, long)]
    list: bool,
    #[arg(short, long)]
    remove: bool,
    #[arg(name = "FILE")]
    file: Option<String>,
}

// TODO: implement better error handling
fn list_crontab() -> Result<String, Box<dyn Error>> {
    let logname = std::env::var("LOGNAME")?;
    let file = format!("/var/spool/cron/{logname}");
    fs::read_to_string(&file).map_err(|x| Box::new(x) as Box<dyn Error>)
}

fn remove_crontab() -> Result<(), Box<dyn Error>> {
    let logname = std::env::var("LOGNAME")?;
    let path = format!("/var/spool/cron/{logname}");
    fs::remove_file(&path).map_err(|x| Box::new(x) as Box<dyn Error>)
}

fn main() {
    let args = CronArgs::parse();


    if args.edit { println!("edit") }

    if args.list {
        match list_crontab() {
            Ok(content) => println!("{}", content),
            Err(_) => {
                println!("No crontab file");
                std::process::exit(1);
            }
        }
    }

    if args.remove {
        match remove_crontab() {
            Ok(()) => println!("Removed crontab file"),
            Err(_) => {
                println!("No crontab file");
                std::process::exit(1);
            },
        }
    }

    if let Some(file) = args.file { println!("FILE = {file}") }
}
