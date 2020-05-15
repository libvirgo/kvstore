use clap::{Arg, App};
use kvstore::utils::config::Config;
use kvstore::utils::protocol::Protocol;

fn main() {
    let matches = App::new("KVStore")
        .version("1.0")
        .author("SuanXuan. <846008723@qq.com>")
        .about("A Simple Distributed Key-Value Store")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config_path")
            .value_name("FILE")
            .help("Sets a custom config file"))
        .get_matches();
    let config = matches.value_of("config").unwrap_or("./src/participant.conf");
    println!("{}", config);
}
