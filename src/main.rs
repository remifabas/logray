mod swgoh;
use reqwest;
use swgoh::Player;

/*async fn loopAlly(ally_codes: Vec<String>) -> Result {
    let call_url: String = String::from("http://api.swgoh.gg/player/");

    for c in ally_codes {
        let mut url = call_url.clone() + &c;
        url.push('/');

        let p: Player = Client::new().get(url).send().await?.json().await?;

        println!("{:#?}", p);
    }
}*/

async fn get_player_info(player_id: &str) -> Result<Player, reqwest::Error> {
    let url = format!("http://api.swgoh.gg/player/{}", player_id);

    let response = reqwest::get(&url).await?;
    let player: Player = response.json().await?;

    Ok(player)
}

#[tokio::main]
async fn main() {
    let ally_codes = vec![
        String::from("616247683"), // Phôenyx
        String::from("327786519"), //Whills Wotan
    ];

    for ally_code in ally_codes {
        match get_player_info(&ally_code).await {
            Ok(p) => {
                println!("{:#?}", p);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
