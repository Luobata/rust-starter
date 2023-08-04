use std::error::Error;

use clap::Parser;
use reqwest::blocking::Client;
use reqwest::header::HeaderMap;


#[derive(Parser)]
#[command(author,version,about = "Sends HTTTP requests and prints detailed information")
]


struct Cli {
    #[arg(short, long, help = "Target URL", required = true)]
    url: String,
}


fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let response = send_request(&cli.url)?;

    print_response_details(response)?;

    Ok(())
}

fn send_request(url: &str) -> Result<reqwest::blocking::Response, Box<dyn Error>> {
    let client = Client::builder().build()?;
    let response = client.get(url).send()?;

    Ok(response)
}

fn print_response_details(response: reqwest::blocking::Response) -> Result<(), Box<dyn Error>> {
    println!("Status: {}", response.status());
    println!("Headers:");
    print_headers(response.headers());

    let body = response.text()?;
    println!("Body:\n{}", body);


    Ok(())
}


fn print_headers(headers: &HeaderMap) {
    for (key , value) in headers.iter() {
        println!(" {}: {}", key, value.to_str().unwrap_or("[unprintable]"));
    }
}
