use clap::Parser;
use kvs::KvStore;

use std::process;

/// An implementation of a simple key-value store
#[derive(Parser, Debug)]
#[clap(
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION")
)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

/// Subcommands supported fot the store
#[derive(Parser, Debug)]
enum SubCommand {
    Set(Set),
    Get(Get),
    Rm(Rm),
}

#[derive(Parser, Debug)]
struct Set {
    key: String,
    value: String,
}

#[derive(Parser, Debug)]
struct Get {
    key: String,
}

#[derive(Parser, Debug)]
struct Rm {
    key: String,
}

fn get(store: KvStore, args: &Get) {
    match store.get(args.key.to_owned()) {
        Ok(v) => {
            match v {
                Some(_) => eprintln!("unimplemented"),
                None => eprintln!("unimplemented"),
            }
        },
        Err(e) => eprintln!("{:?}", e),
    }
    process::exit(1);
}

fn set(mut store: KvStore, args: &Set) {
    match store.set(args.key.to_owned(), args.value.to_owned()) {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("{:?}", e);
            process::exit(1);
        },
    }
}

fn rm(mut store: KvStore, args: &Rm) {
    match store.remove(args.key.to_owned()) {
        Ok(_) => eprintln!("unimplemented"),
        Err(e) => eprintln!("{:?}", e),
    }
    process::exit(1);
}

fn main() {
    let opts: Opts = Opts::parse();
    let store = KvStore::new();

    // 什么鬼，unimplemented 必须全小写，首字母大写都不行吗orz
    match opts.subcmd {
        SubCommand::Set(ref args) => set(store, args),
        SubCommand::Get(ref args) => get(store, args),
        SubCommand::Rm(ref args) => rm(store, args),
    };
}
