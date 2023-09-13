use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version)]
struct Cli {
    #[command(subcommand)]
    args: RDL,
}

#[derive(Subcommand)]
enum RDL {
    Test {
        #[clap(default_value = "")]
        url: String,
    },
    Debug {
        #[clap(default_value = "")]
        url: String,
    },
    Get {
        #[clap(default_value = "")]
        url: String,
    },
}


fn main() {
    let cli = Cli::parse();

    let _ = match cli.args {
        RDL::Test { url } => core::down(url),
        RDL::Debug { url } => Ok(core::debug(url)),
        RDL::Get { url } => Ok(core::get_url(&url)),
    };
}
