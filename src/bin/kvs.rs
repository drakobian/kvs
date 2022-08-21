use kvs::KvStore;

use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Command {
    /// Get a value by the given key
    Get {
        /// The key to search for
        #[clap(value_parser)]
        key: String,
    },
    /// Set a value at the given key
    Set {
        /// The key to insert
        #[clap(value_parser)]
        key: String,

        /// The value to insert at the given key
        #[clap(value_parser)]
        value: String,
    },
    /// Remove a value by the given key
    Rm {
        /// The key to remove
        #[clap(value_parser)]
        key: String,
    },
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The command to run
    #[clap(subcommand)]
    cmd: Command,
}

fn main() {
    let args = Args::parse();

    let mut kvs = KvStore::new();

    match args.cmd {
        Command::Get { key } => {
            kvs.get(key);
        }
        Command::Set { key, value } => kvs.set(key, value),
        Command::Rm { key } => kvs.remove(key),
    }
}
