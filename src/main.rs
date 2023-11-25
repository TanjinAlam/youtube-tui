mod banner;
mod helper;

use banner::BANNER;
use clap::{Command, Arg, ArgAction};

#[macro_use]
extern crate dotenv_codegen;


use crate::helper::{get_search_songs, play_song};
#[tokio::main]
async fn main() {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .before_help(BANNER)
    .arg(Arg::new("song")
    .alias("alias")
    .short('s')
    .long("song")
    .help("Search song (try with '-s neshar bojha')")
    .value_name("for searching songs")
    .action(ArgAction::Set))
    .arg(Arg::new("play")
    .short('p')
    .long("play")
    .hide_short_help(true)
    .help("Play song (try with '-p TUVcZfQe-Kw')")
    .value_name("for playing songs")
    .action(ArgAction::Set))
    
    
    .get_matches(); // builds the instance of ArgMatches

    // to get information about the "cfg" argument we created, such as the value supplied we use
    // various ArgMatches methods, such as [ArgMatches::get_one]
    if let Some(c) = matches.get_one::<String>("song") {
        println!("Your are looking for {c}");
        let _ = get_search_songs(c, dotenv!("API_KEY")).await;
    }
    if let Some(c) = matches.get_one::<String>("play") {
        let _ = play_song(c).await;
    }
}


