use anyhow::{anyhow, Result};
use clap::{Parser};
use reqwest::Url;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct  Opts {
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
        Ok(
            (
            split.next().ok_or_else(err)?.to_string(),
            split.next().ok_or_else(err)?.to_string(),
            )
        )
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("{opts:?}");
}
