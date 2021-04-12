/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

use std::fs;
use std::fs::File;
use std::io::Write;
use std::process;

use aws_hyper::SdkError;
use kms::error::{ReEncryptError, ReEncryptErrorKind};
use kms::Blob;
use kms::Client;
use kms::Region;

use structopt::StructOpt;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::fmt::SubscriberBuilder;

async fn display_error_hint(client: &Client, err: ReEncryptError) {
    eprintln!("Error while decrypting: {}", err);
    match err.kind {
        ReEncryptErrorKind::NotFoundError(_) => {
            let existing_keys = client
                .list_keys()
                .send()
                .await
                .expect("failure to list keys");
            let existing_keys = existing_keys
                .keys
                .unwrap_or_default()
                .into_iter()
                .map(|key| key.key_id.expect("keys must have ids"))
                .collect::<Vec<_>>();
            eprintln!(
                "  hint: Did you create the key first?\n  Existing keys in this region: {:?}",
                existing_keys
            )
        }
        _ => (),
    }
}

#[derive(Debug, StructOpt)]
struct Opt {
    /// The region
    #[structopt(default_value = "us-west-2", short, long)]
    region: String,
    /// The source (original) encryption key
    #[structopt(short, long)]
    source: String,
    /// The destination (new) encryption key
    #[structopt(short, long)]
    destination: String,
    /// The name of the input file containing the text to reencrypt
    #[structopt(short, long)]
    input: String,
    /// The name of the output file containing the reencrypted text
    #[structopt(short, long)]
    output: String,
    /// Whether to display additonal runtime information
    #[structopt(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    if opt.verbose {
        println!("Running ReEncryptData with args:");
        println!("Region:             {}", opt.region);
        println!("Source key ID:      {}", opt.source);
        println!("Destination key ID: {}", opt.destination);
        println!("Input filename:     {}", opt.input);
        println!("Output filename:    {}", opt.output);

        SubscriberBuilder::default()
            .with_env_filter("info")
            .with_span_events(FmtSpan::CLOSE)
            .init();
    }

    let o = &opt.output;

    let config = kms::Config::builder()
        .region(Region::new(opt.region))
        .build();
    let client = kms::Client::from_conf_conn(config, aws_hyper::conn::Standard::https());

    // Get blob from input file
    // Open input text file and get contents as a string
    // input is a base-64 encoded string, so decode it:
    let data = fs::read_to_string(opt.input)
        .map(|input| base64::decode(input).expect("invalid base 64"))
        .map(Blob::new);

    let resp = match client
        .re_encrypt()
        .ciphertext_blob(data.unwrap())
        .source_key_id(opt.source)
        .destination_key_id(opt.destination)
        .send()
        .await
    {
        Ok(output) => output,
        Err(SdkError::ServiceError { err, .. }) => {
            display_error_hint(&client, err).await;
            process::exit(1);
        }
        Err(other) => {
            eprintln!("Encryption failure: {}", other);
            process::exit(1);
        }
    };

    // Did we get an encrypted blob?
    let blob = resp.ciphertext_blob.expect("Could not get encrypted text");
    let bytes = blob.as_ref();

    let s = base64::encode(&bytes);

    let mut ofile = File::create(o).expect("unable to create file");
    ofile.write_all(s.as_bytes()).expect("unable to write");

    if opt.verbose {
        println!("Wrote the following to {}:", o);
        println!("{}", s);
    } else {
        println!("Wrote base64-encoded output to {}", opt.output);
    }
}
