use crate::reqwest;
use std::io::copy;
use std::fs::File;

#[tokio::main]
async fn main() ->Result<()>{
    let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    let body = reqwest::get("https://www.rust-lang.org").await?
        .text().await?;


}