mod swgoh;
use swgoh::Player;

async fn get_player_info(player_id: &str) -> Result<Player, reqwest::Error> {
    let url = format!("http://api.swgoh.gg/player/{}", player_id);

    let response = reqwest::get(&url).await?;
    let player: Player = response.json().await?;

    Ok(player)
}

#[tokio::main]
async fn main() {
    let ally_codes = vec![
        String::from("616247683"), //   Phôenyx
        String::from("327786519"), //   Whills Wotan
    ];

    let mut players: Vec<swgoh::Player> = vec![];

    for ally_code in ally_codes {
        match get_player_info(&ally_code).await {
            Ok(p) => {
                println!("{:#?}", p);
                players.push(p);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
