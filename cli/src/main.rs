//mod migrate;

//use clap::{Args, Parser, Subcommand};
use console::style;
use sqlx::migrate::Migrator;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use ydb_sqlx::connection::{YdbConnectOptions, YdbConnection};
use sqlx::ConnectOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    init_logs();

    if let Err(error) = run().await {
        println!("{} {}", style("error:").bold().red(), error);
        std::process::exit(1);
    }
    
    Ok(())
}
// #[derive(clap::Parser)]
// #[command(author, version, about, long_about = None)]
// struct Cli{
//     cmd: Commands
// }

// #[derive(clap::Subcommand, Debug, Clone)]
// enum Commands{
//     Migrate(MigrateCommand),
// }

// #[derive(Args, Debug, Clone)]
// struct MigrateCommand{
//     sub: MigrateSubcommand,
// }

// #[derive(Subcommand, Debug, Clone)]
// enum MigrateSubcommand{
//     Add{
//         source: String,
//         description: String,
//         reversible: bool,
//         sequential: bool,
//         timestamp: bool
//     },
//     // Run{
//     //     source: String,
//     //     dry_run: bool,
//     //     ignore_missing: bool,
//     //     connect_opts: ConnectOptions,
//     //     target_version: Option<String>
//     // },
//     // Revert{
//     //     source: String,
//     //     dry_run: bool,
//     //     ignore_missing: bool,
//     //     connect_opts: ConnectOptions,
//     // }
// }

async fn run()-> Result<(), Box<dyn std::error::Error>> {

    // let cli = Cli::parse();

    // match cli.cmd {
    //     Commands::Migrate(args) => migrate(args)?
    // }


    let opts = YdbConnectOptions::from_env()?;
    let mut conn = opts.connect().await?;
   
    let path = std::path::Path::new("./migrations");
    let migrator = Migrator::new(path).await?;
    migrator.run_direct(&mut conn).await?;
    Ok(())
}

// fn migrate(args: MigrateCommand)-> Result<(), Box<dyn std::error::Error>> {
//     match args.sub {
//         MigrateSubcommand::Add { source, description, reversible, sequential, timestamp } => {
//             migrate::add(&source, &description, reversible, sequential, timestamp);  
//         },
//     }
//     Ok(())
// }



fn init_logs() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
}

// async fn connect(connect_opts: &YdbConnectOptions) ->  sqlx::Result<YdbConnection> {
//     connect_opts.connect().await
// }
