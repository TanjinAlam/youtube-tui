mod banner;
use banner::BANNER;
use clap::{Command, Arg, ArgAction};
fn main() {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .before_help(BANNER)
    .arg(Arg::new("song name")
    .short('c')
    .action(ArgAction::Set))
    .get_matches(); // builds the instance of ArgMatches

    // to get information about the "cfg" argument we created, such as the value supplied we use
    // various ArgMatches methods, such as [ArgMatches::get_one]
    if let Some(c) = matches.get_one::<String>("song name") {
        println!("Your are looking for {c}");
    }
}
