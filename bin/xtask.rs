/*
    Appellation: xtask <module>
    Contrib: @FL03
*/

use scsys_xtask::cli::Cli;

fn main() {
    tracing_subscriber::fmt().with_max_level(tracing::Level::TRACE).init();
    // This is a placeholder for the main function of the xtask binary.
    // You can add your task implementation here.
    tracing::info!("initialize the tracing modules;");
    // parse the command line arguments
    let cli = Cli::parse();

    tracing::info!("cli: {:#?}", cli);

}