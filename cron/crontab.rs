use clap::Parser;

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

fn main() {
    let args = CronArgs::parse();

    if args.edit { println!("edit") }
    if args.list { println!("list") }
    if args.remove { println!("remove") }
    if let Some(file) = args.file { println!("FILE = {file}") }
}
