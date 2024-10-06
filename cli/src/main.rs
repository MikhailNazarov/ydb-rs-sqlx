use clap::Parser;
use console::style;
use sqlx_cli::Opt;

use std::sync::Once;

use clap::Parser;
use console::style;
use sqlx::any::install_drivers;
use sqlx_cli::Opt;

pub fn install_driver() {
    static ONCE: Once = Once::new();

    ONCE.call_once(|| {
        install_drivers(&[
            ydb_sqlx::any::DRIVER,
        ])
        .expect("drivers already installed")
    });
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    install_driver();
    
    // no special handling here
    if let Err(error) = sqlx_cli::run(Opt::parse()).await {
        println!("{} {}", style("error:").bold().red(), error);
        std::process::exit(1);
    }
}