mod swgoh;
use csv::{QuoteStyle, WriterBuilder};
use swgoh::Player;

async fn get_player_info(player_id: &str) -> Result<Player, reqwest::Error> {
    let url = format!("http://api.swgoh.gg/player/{}", player_id);

    let response = reqwest::get(&url).await?;
    let player: Player = response.json().await?;

    println!("{}\t\t ok", player.datas.name);
    Ok(player)
}

#[tokio::main]
async fn main() {
    let lograys = swgoh::logray::get_lograys_player();
    let mut players: Vec<swgoh::Player> = vec![];

    for ally_code in &lograys {
        match get_player_info(&ally_code.ally_code).await {
            Ok(p) => {
                players.push(p);
            }
            Err(e) => {
                eprintln!("Error: {}, for {:#?}", e, ally_code);
            }
        }
    }

    let mut map_speed = swgoh::units::all_unit(lograys.len());
    let mut map_gear = swgoh::units::all_unit(lograys.len());
    let mut map_health = swgoh::units::all_unit(lograys.len());
    let mut map_ship = swgoh::units::all_ship(lograys.len());

    for (x, p) in players.into_iter().enumerate() {
        for u in &p.units {
            map_speed.retain(|k, v| {
                if &u.unit_data.name == k {
                    v[x] = u.unit_data.stats.speed.to_string();
                }
                true
            });
            map_gear.retain(|k, v| {
                if &u.unit_data.name == k {
                    let mut val = u.unit_data.relic_tier + 11;
                    if val == 12 {
                        val = u.unit_data.gear_level;
                    }
                    v[x] = val.to_string();
                }
                true
            });
            map_health.retain(|k, v| {
                if &u.unit_data.name == k {
                    v[x] = u.unit_data.stats.health.to_string();
                }
                true
            });
            map_ship.retain(|k, v| {
                if &u.unit_data.name == k {
                    v[x] = u.unit_data.rarity.to_string();
                }
                true
            })
        }
    }

    let mut writer_speed = WriterBuilder::new()
        .has_headers(false)
        .quote_style(QuoteStyle::Never)
        .from_path("speed.csv")
        .expect("Failed to create CSV writer");

    let mut writer_ship = WriterBuilder::new()
        .has_headers(false)
        .quote_style(QuoteStyle::Never)
        .from_path("ship.csv")
        .expect("Failed to create CSV writer");

    let mut writer_gear = WriterBuilder::new()
        .has_headers(false)
        .quote_style(QuoteStyle::Never)
        .from_path("relic.csv")
        .expect("Failed to create CSV writer");

    let mut writer_health = WriterBuilder::new()
        .has_headers(false)
        .quote_style(QuoteStyle::Never)
        .from_path("health.csv")
        .expect("Failed to create CSV writer");

    swgoh::csv::write_to_csv(map_speed, &lograys, &mut writer_speed);
    writer_speed.flush().expect("Failed to flush writer");

    swgoh::csv::write_to_csv(map_gear, &lograys, &mut writer_gear);
    writer_gear.flush().expect("Failed to flush writer");

    swgoh::csv::write_to_csv(map_health, &lograys, &mut writer_health);
    writer_health.flush().expect("Failed to flush writer");

    swgoh::csv::write_to_csv(map_ship, &lograys, &mut writer_ship);
    writer_ship.flush().expect("Failed to flush writer");

    println!("{} players handled", lograys.len());
}
