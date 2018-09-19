#[macro_use] extern crate log;
extern crate clap;
use clap::{Arg, App};
extern crate loggerv;


fn main() {
    let args = App::new("raw image development")
        .version("0.0.1")
        .author("Nobuyuki Horiuchi <horiuchinobuyuki@gmail.com>")
        .about("wip..")
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .value_name("input")
            .help("input file")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .get_matches();

    loggerv::init_with_verbosity(args.occurrences_of("v")).unwrap();

    let input = args.value_of("input").unwrap();
    debug!("input file: {}", input);
}

