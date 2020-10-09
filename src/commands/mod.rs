use std::path::PathBuf;
use structopt::StructOpt;

/// A Sitemap Tool
#[derive(StructOpt, Debug)]
#[structopt(name = "crawler")]
pub(crate) struct Opt {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag. The name of the
    // argument will be, by default, based on the name of the field.
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    /// Set speed
    #[structopt(short ="p", long, default_value = "0")]
    depth: f64,

    /// Output file
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,

    /// admin_level to consider
    #[structopt(short, long)]
    url: Vec<String>,
}
