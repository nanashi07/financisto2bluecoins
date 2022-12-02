use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::str::FromStr;
use std::{
    fs,
    io::{BufReader, Read},
};

use flate2::bufread::GzDecoder;
use log::{info, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Config;

use financisto2bluecoins::*;

#[test]
fn financisto_to_bluecoins() -> Result<()> {
    init_log("INFO")?;

    let filename = "data/20221202_210902_662.backup";
    let file = fs::File::open(filename)?;
    let reader = BufReader::new(file);
    let mut decoded = GzDecoder::new(reader);
    let mut content = String::new();
    decoded.read_to_string(&mut content)?;

    let lines = content
        .split("\n")
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();

    let data = convert_maps(&lines)?;

    let mut statements = Vec::new();
    statements.append(&mut migrate_accounts(&data.accounts, &data.currencies)?);
    statements.append(&mut migrate_categories(&data.categories)?);
    statements.append(&mut migrate_transactions(
        &data.transactions,
        &data.currencies,
    )?);

    if !Path::new("output").exists() {
        fs::create_dir("output")?;
    }
    fs::write("output/bluecoins.sql", statements.join("\n"))?;

    Ok(())
}

fn init_log(level: &str) -> Result<()> {
    let level = LevelFilter::from_str(level).unwrap_or(LevelFilter::Info);

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} [{h({l})}] {m}{n}", // TODO: configurable pattern
        )))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(level))?;

    let _ = log4rs::init_config(config)?;
    Ok(())
}

fn _resolve_fields(list: Vec<&HashMap<String, String>>) {
    let mut types: HashMap<String, HashSet<String>> = HashMap::new();

    for map in list {
        let entity = map.get("entity").unwrap();
        if let Some(set) = types.get_mut(entity) {
            for key in map.keys() {
                set.insert(key.to_owned());
            }
        } else {
            let mut set: HashSet<String> = HashSet::new();
            for key in map.keys() {
                set.insert(key.to_owned());
            }
            types.insert(entity.to_owned(), set);
        }
    }

    info!("{:?}", types);
}
