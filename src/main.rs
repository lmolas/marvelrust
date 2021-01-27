use clap::{Arg, App, SubCommand};
use std::io::Read;
use std::time::{SystemTime, UNIX_EPOCH};
use md5;
use toml;
use std::fs;
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct Config {
    api: Api,
}

#[derive(Deserialize)]
struct Api {
    public_key: String,
    private_key: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new("marvelrust")
        .about("Cli to call marvel Rest api")
        .version("v0.0.1")
        .author("Lionel M.")
        .subcommand(SubCommand::with_name("characters")
            .about("Fetch character resource")
            .arg(Arg::with_name("character_name")
                .short("n")
                .long("character_name")
                .help("character name")
                .value_name("character_name")));

    let matches = app.get_matches();

    let config = get_config()?;

    if let Some(matches) = matches.subcommand_matches("characters") {
        get_characters(config, matches.value_of("character_name"))?;
    }

    Ok(())
}

fn get_characters(config: Config, character_name: Option<&str>) -> Result<(), Box<dyn std::error::Error>>  {
    let ts = get_timestamp()?;

    let auth = format!("{}{}{}", ts, config.api.private_key, config.api.public_key);

    let hash = md5::compute(auth);

    let options = match character_name { Some(name) => format!("&name={}", name), _ => String::from("") };
    let url = format!("https://gateway.marvel.com:443/v1/public/characters?ts={}&apikey={}&hash={:x}{}", ts, config.api.public_key, hash, options);

    let mut res = reqwest::blocking::get(&url)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    //println!("Status: {}", res.status());
    //println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);

    Ok(())
}

fn get_timestamp() -> Result<u128, Box<dyn std::error::Error>> {
    let start = SystemTime::now();
    let duration = start.duration_since(UNIX_EPOCH)?;

    Ok(duration.as_millis())
}

fn get_config() -> Result<Config, Box<dyn std::error::Error>> {
    let file = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&file)?;
    Ok(config)
}
