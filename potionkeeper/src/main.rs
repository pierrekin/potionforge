extern crate clap;
extern crate potionforge;

mod debug;
mod printer;
mod recommend;

use clap::{arg, command, Command};

fn main() {
    let matches = command!()
        .subcommand(
            Command::new("recommend")
                .about("Recommend a potion")
                .arg(
                    arg!(-l --"cbc-loglevel" <STRING> "CBC log level")
                        .default_value("0")
                        .required(false),
                )
                .arg(
                    arg!(-c --config <PATH> "Config file")
                        .default_value("config.yml")
                        .required(false),
                ),
        )
        .subcommand(
            Command::new("debug").about("Debug a potion").arg(
                arg!(-c --config <PATH> "Config file")
                    .default_value("debug.yml")
                    .required(false),
            ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("recommend") {
        let config_file = matches.get_one::<String>("config").unwrap();
        let cbc_loglevel = matches.get_one::<String>("cbc-loglevel").unwrap();
        recommend::recommend(config_file.clone(), cbc_loglevel.clone());
    }

    if let Some(matches) = matches.subcommand_matches("debug") {
        let config_file = matches.get_one::<String>("config").unwrap();
        debug::debug(config_file.clone());
    }
}
