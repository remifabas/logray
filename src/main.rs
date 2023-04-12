use log::*;
use reqwest::{Client, Error};

mod swgoh;
use swgoh::Player;
#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let ally_codes = vec![
        // String::from("616247683"), // Phôenyx
        String::from("327786519"), //Whills Wotan
    ];

    loopAlly(ally_codes).await;

    match loopAlly(ally_codes).await {
        Ok(_) => Ok("Done !"),
        Err(e) => error!("An error ocurred: {}", e),
    };

    Ok(())
}

async fn loopAlly(ally_codes: Vec<String>) -> Result<(), Error> {
    let call_url: String = String::from("http://api.swgoh.gg/player/");

    for c in ally_codes {
        let mut url = call_url.clone() + &c;
        url.push('/');

        let p: Player = Client::new().get(url).send().await?.json().await?;

        println!("{:#?}", p);
    }

    Ok(())
}
