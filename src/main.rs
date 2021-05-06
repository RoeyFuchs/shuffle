use std::io::Read;
use std::process;

use shuffle::config::Config;
use shuffle::opt::Opt;
use shuffle::string_manipulation::{separat_string, shuffle_vec, write_to_stream};
use structopt::StructOpt;
fn main() {
    let args = Opt::from_args();

    let mut conf: Config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });



    let mut buffer = String::new();

    conf.source
        .read_to_string(&mut buffer)
        .unwrap_or_else(|err| {
            eprintln!("Error while reading stream: {}", err);
            process::exit(1);
        });

    let mut lines = separat_string(&buffer, &conf.separator);
    shuffle_vec(&mut lines);

    write_to_stream(&lines, &mut conf.destinition).unwrap_or_else(|err| {
        eprintln!("Error while writing to stream: {}", err);
        process::exit(1);
    });
}
