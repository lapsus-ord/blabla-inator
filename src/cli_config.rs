use clap::Parser;
use clap_verbosity_flag::Verbosity;
use tracing_log::AsTrace;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub server: String,

    #[arg(short, long)]
    pub topic: String,

    #[command(flatten)]
    pub verbose: Verbosity,
}

impl Cli {
    pub fn init_logging(&self) {
        tracing_subscriber::fmt()
            .with_max_level(self.verbose.log_level_filter().as_trace())
            .init();
    }
}
