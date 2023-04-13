mod swgoh;
use csv::{Writer, WriterBuilder};
use std::collections::HashMap;
use std::fs::File;
use swgoh::Player;

async fn get_player_info(player_id: &str) -> Result<Player, reqwest::Error> {
    let url = format!("http://api.swgoh.gg/player/{}", player_id);

    let response = reqwest::get(&url).await?;
    let player: Player = response.json().await?;

    Ok(player)
}

fn write_to_csv_file(
    map: HashMap<String, Vec<String>>,
    guild_member: Vec<String>,
    writer: &mut Writer<File>,
) {
    if let Err(e) = writer.write_record(&[guild_member]) {
        eprintln!("Error writing to CSV file: {}", e);
    }

    for (x, p) in map.into_iter() {
        if let Err(e) = writer.write_record(&[p.datas.arena_rank.to_string()]) {
            eprintln!("Error writing to CSV file: {}", e);
        }
    }

    /*
    if let Err(e) = writer.write_record(&[h, p.datas.arena_rank.to_string()]) {
        eprintln!("Error writing to CSV file: {}", e);
    }
    */
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

    let mut names = swgoh::units::all_unit(3);

    for (x, p) in players.into_iter().enumerate() {
        for u in &p.units {
            names.retain(|k, v| {
                if &u.unit_data.name == k {
                    v[x] = u.unit_data.stats.speed.to_string();
                }
                true
            })
        }
    }
    //println!("{:#?}", names);
    write_to_csv_file(names, ally_codes, &mut writer);
    writer.flush().expect("Failed to flush CSV writer");
}
