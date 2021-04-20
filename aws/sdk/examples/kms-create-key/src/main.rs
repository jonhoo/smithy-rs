/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */
use std::process;

use kms::{Client, Config, Region};

use aws_types::region::{EnvironmentProvider, ProvideRegion};

use structopt::StructOpt;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::fmt::SubscriberBuilder;

#[derive(Debug, StructOpt)]
struct Opt {
    /// The region
    #[structopt(short, long)]
    region: Option<String>,

    /// Activate verbose mode    
    #[structopt(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() {
    let Opt { region, verbose } = Opt::from_args();

    let region = EnvironmentProvider::new()
        .region()
        .or_else(|| region.as_ref().map(|region| Region::new(region.clone())))
        .unwrap_or_else(|| Region::new("us-west-2"));

    if verbose {
        println!("KMS client version: {}\n", kms::PKG_VERSION);
        println!("Region:      {:?}", &region);

        SubscriberBuilder::default()
            .with_env_filter("info")
            .with_span_events(FmtSpan::CLOSE)
            .init();
    }

    let config = Config::builder().region(region).build();

    let client = Client::from_conf(config);

    match client.create_key().send().await {
        Ok(resp) => {
            let id = resp
                .key_metadata
                .unwrap()
                .key_id
                .unwrap_or_else(|| String::from("No ID!"));
            println!("Key: {}", id);
        }
        Err(_) => {
            println!();
            process::exit(1);
        }
    };
    println!();
}
