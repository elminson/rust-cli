use blkrs::run_lsblk;
use clap::ColorChoice;
use clap::{Arg, Command};
use blkrs::read_dir;

fn main() {


    let matches = Command::new("lsblk")
        .version("0.0.1")
        .author("Alfredo Deza")
        .about("lsblk in Rust")
        .color(ColorChoice::Always)
        .arg(
            Arg::new("path")
                .help("Device to query")
                .required(true)
                .index(1),
        )
        .get_matches();

    if let Some(path) = matches.get_one::<String>("path") {
        let output = serde_json::to_string(&read_dir(&path));
        match output {
            Ok(output) => println!("{}", output),
            Err(error) => {
                eprintln!("Error: {}", error);
            }
        }
    } else {
        println!("No valid path provided");
    }
}
