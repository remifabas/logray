use reqwest::{Client, Error};

mod swgoh;
use swgoh::Player;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Hello, world!");

    let p: Player = Client::new()
        .get("http://api.swgoh.gg/player/327786519/")
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", p);

    Ok(())
}
