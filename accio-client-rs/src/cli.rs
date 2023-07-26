use crate::command::Command;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct CLI {
    #[clap(subcommand)]
    pub command: Command,
}
