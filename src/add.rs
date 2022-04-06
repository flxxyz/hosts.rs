use clap::{Arg, ArgMatches, Command};

pub const COMMAND_NAME: &str = "add";

pub fn command() -> Command<'static> {
    let args_url = Arg::new("url")
        .help("hosts的url")
        .takes_value(true)
        .multiple_values(true)
        .required(true);
    let args_help = Arg::new("help").short('h').long("help").help("帮助信息");

    Command::new(COMMAND_NAME)
        .about("添加hosts网络资源路径")
        .alias("a")
        .arg(args_url)
        .arg(args_help)
}

pub fn handle(matches: &ArgMatches) -> &ArgMatches {
    if matches.occurrences_of("url") > 0 {
        let urls: Vec<_> = matches.values_of("url").unwrap().collect();
        for url in urls.iter() {
            core::insert(url);
        }
    }

    return matches;
}
