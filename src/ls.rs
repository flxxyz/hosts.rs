use clap::{Arg, ArgMatches, Command};

pub const COMMAND_NAME: &str = "list";

pub fn command() -> Command<'static> {
    let args_help = Arg::new("help").short('h').long("help").help("帮助信息");

    Command::new(COMMAND_NAME)
        .about("列出添加过的hosts")
        .aliases(&[&"ls", "l"])
        .arg(args_help)
}

pub fn handle(matches: &ArgMatches) -> &ArgMatches {
    let urls = core::find();
    for url in urls.iter() {
        println!("{}", url);
    }
    return matches;
}
