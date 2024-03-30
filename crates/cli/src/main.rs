use clap::{arg, Command};
use libruntime::context::default;
use libruntime::create::{create, CreateBuilder};
use libruntime::log::init_logger;
use log::{error, info};
use std::process::exit;

fn cli() -> Command {
    Command::new("balaeno")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("create")
                .about("create container")
                .arg(arg!(<container_id>))
                .arg(arg!(<path_to_bundle>))
                .arg_required_else_help(true),
        )
}

fn main() {
    init_logger();

    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("create", sub_matches)) => {
            let path_to_bundle = sub_matches
                .get_one::<String>("path_to_bundle")
                .map(|s| s.as_str())
                .unwrap();
            let container_id = sub_matches
                .get_one::<String>("container_id")
                .map(|s| s.as_str())
                .unwrap();
            match create(
                default(),
                CreateBuilder::new(path_to_bundle.to_string(), container_id.to_string()),
            ) {
                Ok(_) => {
                    info!("container created");
                }
                Err(e) => {
                    error!("failed to create container: {:?}", e);
                    exit(1);
                }
            }
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    }
}
