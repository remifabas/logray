use std::collections::HashMap;

pub fn all_unit(len: usize) -> HashMap<String, Vec<String>> {
    HashMap::from([
        (
            String::from("Jedi Master Kenobi"),
            vec!["".to_string(); len],
        ),
        (
            String::from("Jedi Master Luke Skywalker"),
            vec!["".to_string(); len],
        ),
        (String::from("Lord Vader"), vec!["".to_string(); len]),
        (String::from("Jabba the Hutt"), vec!["".to_string(); len]),
    ])
}
