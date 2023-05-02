use super::Allies;

use csv::Writer;
use std::fs::File;

/*pub fn write_datas(mut names: BTreeMap<String, Vec<String>>, guild_members: Vec<Allies>) {
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

    let mut writer = WriterBuilder::new()
        .has_headers(false)
        .quote_style(QuoteStyle::Never)
        .from_path("output.csv")
        .expect("Failed to create CSV writer");

    let mut writer_2 = WriterBuilder::new()
        .has_headers(false)
        .quote_style(QuoteStyle::Never)
        .from_path("charac.csv")
        .expect("Failed to create CSV writer");

    write_to_csv_speed(names, guild_members, &mut writer);
}*/

pub fn write_to_csv(
    map: indexmap::IndexMap<String, Vec<String>>,
    guild_members: Vec<Allies>,
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
