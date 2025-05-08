use clap::Parser;
use clap_verbosity::Verbosity;

#[derive(Debug, Parser)]
pub struct Options {
    #[clap(
        short,
        long,
        default_value_t = 8000,
        help = "Port to run the server on"
    )]
    pub port: u16,

    #[clap(flatten)]
    pub verbosity: Verbosity,
}
