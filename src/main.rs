mod swgoh;
use csv::{Writer, WriterBuilder};
//use std::collections::HashMap;
use std::fs::File;
use swgoh::Player;

async fn get_player_info(player_id: &str) -> Result<Player, reqwest::Error> {
    let url = format!("http://api.swgoh.gg/player/{}", player_id);

    let response = reqwest::get(&url).await?;
    let player: Player = response.json().await?;

    Ok(player)
}

fn write_to_csv_file(p: swgoh::Player, writer: &mut Writer<File>) {
    for char in p.units {
        println!("{:#?}", char.unit_data.name);
    }
    if let Err(e) = writer.write_record(&[p.datas.name, p.datas.arena_rank.to_string()]) {
        eprintln!("Error writing to CSV file: {}", e);
    }
}

#[tokio::main]
async fn main() {
    let mut writer = WriterBuilder::new()
        .has_headers(false)
        .from_path("output.csv")
        .expect("Failed to create CSV writer");

    let ally_codes = vec![
        String::from("616247683"), //   Phôenyx
        String::from("327786519"), //   Whills Wotan
        String::from("543168732"), //   M'enfin
    ];

    let mut players: Vec<swgoh::Player> = vec![];

    for ally_code in ally_codes {
        match get_player_info(&ally_code).await {
            Ok(p) => {
                //println!("{:#?}", p);
                players.push(p);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }

    for p in players {
        let mut names = swgoh::units::all_unit();

        // TODO look here
        https://stackoverflow.com/questions/45724517/how-to-iterate-through-a-hashmap-print-the-key-value-and-remove-the-value-in-ru
        for u in p.units {
            for key in names.keys() {
                println!("{key}");
                names.insert(key.to_string(), u.unit_data.relic_tier.into());
            }
        }

        //write_to_csv_file(p, &mut writer);
    }

    writer.flush().expect("Failed to flush CSV writer");
}
