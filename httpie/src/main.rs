use anyhow::{anyhow, Result};
use clap::Parser;
use reqwest::{header, Client, Response, Url};
use std::collections::HashMap;
use colored::*;
use mime::Mime;

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
    Ok(print_resp(resp).await?)
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for (key, value) in args.body.iter() {
        body.insert(key, value);
    }
    let resp = client.post(&args.url).json(&body).send().await?;
    Ok(print_resp(resp).await?)
}

fn print_status(resp: &Response) { 
    let status = format!("{:?} {}", resp.version(), resp.status()).blue(); 
    println!("{}\n", status);
}

fn print_headers(resp: &Response) {    
    for (name, value) in resp.headers() {
        println!("{}: {:?}", name.to_string().green(), value);    
    }    
    print!("\n");
}

fn print_body(m: Option<Mime>, body: &String) {    
    match m {       
    Some(v) if v == mime::APPLICATION_JSON => {            
        println!("{}", jsonxf::pretty_print(body).unwrap().cyan())        
    }       
    _ => println!("{}", body),    
    }
}

fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers().get(header::CONTENT_TYPE).map(|v| v.to_str().unwrap().parse().unwrap())
}

async fn print_resp(resp: Response) -> Result<()> {
    print_status(&resp);    
    print_headers(&resp);    
    let mime = get_content_type(&resp);    
    let body = resp.text().await?;    
    print_body(mime, &body);    
    Ok(())
}


#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let mut headers = header::HeaderMap::new();
    headers.insert("X-POWERED-BY", "Rust".parse()?);
    headers.insert(header::USER_AGENT, "Rust Httpie".parse()?);
    let client = Client::builder().default_headers(headers).build()?;
    let result = match opts.subcmd {
        SubCommand::Get(ref args) => get(client, args).await?,
        SubCommand::Post(ref args) => post(client, args).await?,
    };
    Ok(result)
}
