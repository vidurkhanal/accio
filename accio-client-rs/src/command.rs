use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Command {
    Ping,
    Get { key: String },
    Del { key: String },
    Set { key: String, val: String },
    Invalid,
}
