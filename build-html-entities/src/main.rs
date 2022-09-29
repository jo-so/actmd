use serde::Deserialize;

use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{
        self,
        Read,
    },
};

#[derive(Deserialize)]
struct Entry {
    characters: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = clap::Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!("\n"))
        .about(clap::crate_description!())
        .arg(clap::Arg::new("file")
             .value_name("INPUT")
             .help("File with CommonMark Spec"),
        )
        .get_matches();

    let mut input = String::new();
    if let Some(file) = args.get_one("file").map(|name: &&str| File::open(name)) {
        file?.read_to_string(&mut input)?;
    } else {
        io::stdin().read_to_string(&mut input)?;
    }

    let data = serde_json::from_str::<HashMap<String, Entry>>(&input)?;
    let mut data = data.iter().collect::<Vec<_>>();
    data.sort_by_key(|e| e.0);

    println!("[");
    for (key, val) in data {
        if !key.starts_with("&") || !key.ends_with(";") {
            continue;
        }

        println!(
            r#"  ("{}", "{}"),"#,
            &key[1..key.len() - 1],
            val.characters.escape_default()
        );
    }
    println!("]");

    Ok(())
}
