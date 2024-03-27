use clap::{arg, Command};

fn cli() -> Command {
    Command::new("balaeno")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("create")
                .about("create container")
                .arg(arg!([bundle]))
                .arg(arg!([id]))
                .arg_required_else_help(true),
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("create", sub_matches)) => {
            let bundle = sub_matches
                .get_one::<String>("bundle")
                .map(|s| s.as_str())
                .unwrap();
            let id = sub_matches
                .get_one::<String>("id")
                .map(|s| s.as_str())
                .unwrap();
            println!("create container: bundle: {:?}, id: {:?}", bundle, id);
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    }
}
