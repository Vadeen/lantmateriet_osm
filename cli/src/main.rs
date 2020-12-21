extern crate vadeen_osm;

use cli::Cli;
use lantmateriet::Lantmateriet;
use vadeen_osm::osm_io::error::Error;
use vadeen_osm::osm_io::write;

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}

fn run() -> std::result::Result<(), Error> {
    let cli = Cli::run();

    let lantmateriet = Lantmateriet::open(&cli.shape_folder, &cli.config, &cli.region)?;
    let osm = lantmateriet.read()?;

    println!("Writing {}", &cli.output);
    write(cli.output, &osm)?;

    Ok(())
}
