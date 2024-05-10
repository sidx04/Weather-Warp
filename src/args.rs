use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about = "Get weather information")]
pub struct Args {
    #[clap(subcommand)]
    pub command: Option<Command>,
}

#[derive(Parser, Debug)]
pub enum Command {
    /// Get weather information
    Weather {
        /// City name
        #[clap(
            short = 'c',
            long = "city",
            help = "Set the city (eg: Delhi, Kochi, Chicago)"
        )]
        city: Option<String>,
        /// Country code
        #[clap(
            short = 'C',
            long = "country",
            help = "Set the country code (eg: IND for India, US for United States)"
        )]
        country: Option<String>,
    },
    /// Set default city and country configuration
    Config {
        /// City name
        #[clap(short = 'c', long = "city", help = "Set the city")]
        city: Option<String>,
        /// Country code
        #[clap(
            short = 'C',
            long = "country",
            help = "Set the country code (eg: IND for India)"
        )]
        country: Option<String>,
    },
}

impl Args {
    pub fn arg_parse() -> Self {
        Args::parse()
    }
}
