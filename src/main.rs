use server::serve;
use log::{debug};

// CLI Option Parsing Stuff
use structopt::StructOpt;
#[derive(StructOpt, Debug)]
#[structopt(name = "solid-rust")]
struct CliOpts {
    #[structopt(short, long, default_value = "7070")]
    port: u16,
}

// *** *** ***
// ENTRY POINT
// *** *** ***
#[tokio::main]
async fn main() /*-> Result<(), Box<dyn std::error::Error>>*/ {
    pretty_env_logger::init();

    let cli_opts = CliOpts::from_args();
    debug!("{:?}", cli_opts);

    // todo: Handle graceful shutdowns somehow
    // let (tx, rx) = tokio::sync::oneshot::channel();

    serve(cli_opts.port).await

}