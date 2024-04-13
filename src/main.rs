use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Select};

use agscheduler_cli::AGScheduler;

/// Command line interface for AGScheduler
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// AGScheduler HTTP endpoint
    #[arg(short, long, default_value = "http://127.0.0.1:36370")]
    endpoint: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!("Connecting to `{}`...", args.endpoint);

    let ags = AGScheduler {
        endpoint: args.endpoint,
    };

    loop {
        let selections = &[
            "Get Info",
            "Get Funcs",
            "Get Job",
            "Get All Jobs",
            "Delete Job",
            "Delete All Jobs",
            "Pause Job",
            "Resume Job",
            "Start",
            "Stop",
            "Get Cluster Nodes",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select your operation")
            .default(0)
            .max_length(4)
            .items(&selections[..])
            .interact()
            .unwrap();

        match selection {
            0 => ags.get_info().await,
            1 => ags.get_funcs().await,
            2 => ags.get_job().await,
            3 => ags.get_all_jobs().await,
            4 => ags.delete_job().await,
            5 => ags.delete_all_jobs().await,
            6 => ags.pause_or_resume_job("pause").await,
            7 => ags.pause_or_resume_job("resume").await,
            8 => ags.start_or_stop("start").await,
            9 => ags.start_or_stop("stop").await,
            10 => ags.get_cluster_nodes().await,
            _ => panic!("Error"),
        };
    }
}
