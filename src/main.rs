mod swgoh;
use csv::{QuoteStyle, Writer, WriterBuilder};
use std::collections::BTreeMap;
use std::fs::File;
use swgoh::Player;

async fn get_player_info(player_id: &str) -> Result<Player, reqwest::Error> {
    let url = format!("http://api.swgoh.gg/player/{}", player_id);

    let response = reqwest::get(&url).await?;
    let player: Player = response.json().await?;

    Ok(player)
}

fn write_to_csv_file(
    map: BTreeMap<String, Vec<String>>,
    guild_members: Vec<swgoh::Allies>,
    writer: &mut Writer<File>,
) {
    let mut lines = Vec::new();

    let mut line_guild_members = String::from("Guild Members");
    for a in guild_members {
        line_guild_members.push_str("; ");
        line_guild_members.push_str(&a.name);
    }

    lines.push(line_guild_members);

    for (current_chara_name, vec_current_char) in map.iter() {
        let mut line_current_character = String::from(current_chara_name);
        for value in vec_current_char {
            line_current_character.push_str("; ");
            line_current_character.push_str(value);
        }
        lines.push(line_current_character);
    }

    for l in lines {
        if let Err(e) = writer.write_record(&[l]) {
            eprintln!("Error writing to CSV file: {}", e);
        }
    }
}

#[tokio::main]
async fn main() {
    let mut writer = WriterBuilder::new()
        .has_headers(false)
        .quote_style(QuoteStyle::Never)
        .from_path("output.csv")
        .expect("Failed to create CSV writer");

    let lograys = swgoh::logray::get_lograys_player();
    let mut players: Vec<swgoh::Player> = vec![];

    for ally_code in &lograys {
        match get_player_info(&ally_code.ally_code).await {
            Ok(p) => {
                //println!("{:#?}", p);
                players.push(p);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }

    let mut names = swgoh::units::all_unit(lograys.len());

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
    write_to_csv_file(names, lograys, &mut writer);
    writer.flush().expect("Failed to flush CSV writer");
}
