use kvs::KvStore;

use clap::{Parser, ValueEnum};

// TODO: Validate that key is always given,
// and that value is given for Set
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Command {
    Get,
    Set,
    Rm,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The command to run
    #[clap(arg_enum, value_parser)]
    cmd: Command,

    /// The key to use
    #[clap(value_parser)]
    key: String,

    /// The value to optionally use
    #[clap(value_parser)]
    value: Option<String>,
}

fn main() {
    let args = Args::parse();

    let mut kvs = KvStore::new();

    match args.cmd {
        Command::Get => {
            kvs.get(args.key);
        }
        Command::Set => kvs.set(args.key, args.value.unwrap()),
        Command::Rm => kvs.remove(args.key),
    }
}
