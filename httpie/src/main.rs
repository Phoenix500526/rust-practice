use anyhow::{anyhow, Result};
use clap::Parser;
use reqwest::{header, Client, Response, Url};
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
}

fn parse_url(url: &str) -> Result<String> {
    let _url: Url = url.parse()?;
    Ok(url.into())
}

#[derive(Parser, Debug)]
struct Get {
    /// HTTP URL
    #[clap(value_parser = parse_url)]
    url: String,
}

#[derive(Parser, Debug)]
struct Post {
    /// HTTP URL
    #[clap(value_parser = parse_url)]
    url: String,
    /// HTTP body
    #[clap(value_parser = parse_kv_pairs)]
    body: Vec<(String, String)>,
}

fn parse_kv_pairs(s: &str) -> Result<(String, String)> {
    let mut split = s.split("=");
    let err = || anyhow!(format!("Failed to parse {s}"));
    Ok((
        split.next().ok_or_else(err)?.to_string(),
        split.next().ok_or_else(err)?.to_string(),
    ))
}

async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    println!("{:?}", resp.text().await?);
    Ok(())
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for (key, value) in args.body.iter() {
        body.insert(key, value);
    }
    let resp = client.post(&args.url).json(&body).send().await?;
    println!("{:?}", resp.text().await?);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let client = Client::new();
    let result = match opts.subcmd {
        SubCommand::Get(ref args) => get(client, args).await?,
        SubCommand::Post(ref args) => post(client, args).await?,
    };
    Ok(result)
}
