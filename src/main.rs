use clap::ColorChoice;
use clap::{Arg, Command};
use read_dir::read_dir;

fn main() {


    let matches = Command::new("read_dir")
        .version("0.0.1")
        .author("Elminson De Oleo Baez")
        .about("read dir in Rust")
        .color(ColorChoice::Always)
        .arg(
            Arg::new("path")
                .help("path to read")
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
