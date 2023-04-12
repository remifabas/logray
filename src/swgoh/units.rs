use std::collections::HashMap;

pub fn all_unit() -> HashMap<String, i32> {
    HashMap::from([
        (String::from("Jedi Master Kenobi"), 0),
        (String::from("Jedi Master Luke Skywalker"), 0),
        (String::from("Lord Vader"), 0),
    ])
}
