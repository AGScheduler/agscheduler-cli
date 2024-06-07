#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Select};

use agscheduler_cli::api_client::AGScheduler;
use agscheduler_cli::interaction::Interaction;

/// Command line interface for AGScheduler
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// AGScheduler HTTP endpoint
    #[arg(short, long, default_value = "http://127.0.0.1:36370")]
    endpoint: String,
    /// SHA256 encrypted authorization password, e.g. here is admin: `echo -n admin | shasum -a 256` -> `8c6976e5b5410415bde908bd4dee15dfb167a9c873fc4bb8a81f6f2ab448a918`
    #[arg(short, long, default_value = "")]
    password_sha2: String,
}

#[tokio::main]
#[cfg_attr(coverage_nightly, coverage(off))]
async fn main() {
    let args = Args::parse();

    println!("Connecting to `{}`...", args.endpoint);

    let ags = AGScheduler {
        endpoint: args.endpoint,
        password_sha2: args.password_sha2,
    };

    loop {
        let selections = &[
            "Add Job",
            "Get Job",
            "Get All Jobs",
            "Update Job",
            "Delete Job",
            "Delete All Jobs",
            "Pause Job",
            "Resume Job",
            "Run Job",
            "Schedule Job",
            "Start",
            "Stop",
            "Get Records",
            "Get All Records",
            "Delete Records",
            "Delete All Records",
            "Get Info",
            "Get Funcs",
            "Get Cluster Nodes",
        ];

        let interaction = Interaction {};

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select your operation")
            .default(0)
            .max_length(8)
            .items(&selections[..])
            .interact()
            .unwrap();

        match selection {
            0 => ags.add_job(&interaction).await,
            1 => ags.get_job(&interaction).await,
            2 => ags.get_all_jobs().await,
            3 => ags.update_job(&interaction).await,
            4 => ags.delete_job(&interaction).await,
            5 => ags.delete_all_jobs(&interaction).await,
            6 => ags.pause_or_resume_job("pause", &interaction).await,
            7 => ags.pause_or_resume_job("resume", &interaction).await,
            8 => ags.run_or_schedule_job("run", &interaction).await,
            9 => ags.run_or_schedule_job("schedule", &interaction).await,
            10 => ags.start_or_stop("start").await,
            11 => ags.start_or_stop("stop").await,
            12 => ags.get_records(&interaction).await,
            13 => ags.get_all_records(&interaction).await,
            14 => ags.delete_records(&interaction).await,
            15 => ags.delete_all_records(&interaction).await,
            16 => ags.get_info().await,
            17 => ags.get_funcs().await,
            18 => ags.get_cluster_nodes().await,
            _ => panic!("Error"),
        };
    }
}
