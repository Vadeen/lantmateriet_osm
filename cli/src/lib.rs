extern crate clap;

use clap::{App, Arg};

pub struct Cli {
    pub shape_folder: String,
    pub output: String,
    pub output_format: Option<String>,
    pub config: Option<String>,
    pub region: Option<String>,
}

impl Cli {
    pub fn run() -> Cli {
        let matches = App::new("Lantmäteriet")
            .version(env!("CARGO_PKG_VERSION"))
            .about("Converts lantmäteriet shape files to osm formats.")
            .arg(
                Arg::with_name("SHAPE_FOLDER")
                    .help("Base folder. E.g. ./terrang/21/")
                    .value_name("SHAPE_FOLDER")
                    .required(true)
                    .help("Sets a custom config file"),
            )
            .arg(
                Arg::with_name("OUTPUT")
                    .help("Output file")
                    .value_name("FILE")
                    .short("o")
                    .long("output")
                    .required(true)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("OUTPUT_FORMAT")
                    .help("Force output format, osm or o5m. Default is based on output file name")
                    .value_name("OUTPUT_FORMAT")
                    .short("f")
                    .long("format")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("REGION")
                    .help("Force region. Default is identified by folder content")
                    .value_name("REGION")
                    .short("r")
                    .long("region")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("CONFIG")
                    .help("Custom config file")
                    .short("c")
                    .long("config")
                    .value_name("FILE")
                    .takes_value(true),
            )
            .get_matches();

        Cli {
            shape_folder: matches.value_of("SHAPE_FOLDER").unwrap().to_owned(),
            output: matches.value_of("OUTPUT").unwrap().to_owned(),
            output_format: matches.value_of("OUTPUT_FORMAT").map(|s| s.to_owned()),
            config: matches.value_of("CONFIG").map(|s| s.to_owned()),
            region: matches.value_of("REGION").map(|s| s.to_owned()),
        }
    }
}
