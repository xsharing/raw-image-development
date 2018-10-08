#[macro_use] extern crate log;
extern crate clap;
extern crate loggerv;
extern crate rawloader;
extern crate image;
extern crate histogram;

mod methods;
use methods::plain;

use clap::{Arg, App};

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
        .arg(Arg::with_name("method")
            .short("m")
            .long("method")
            .value_name("method")
            .help("method")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .get_matches();

    loggerv::init_with_verbosity(args.occurrences_of("v")).unwrap();

    let input = args.value_of("input").unwrap();
    let method = args.value_of("method").unwrap();
    debug!("input file: {}, method: {}", input, method);

    match method.to_string().as_str() {
        "plain" => plain::main_logic(input),
        _ => plain::main_logic(input)
    }
}

