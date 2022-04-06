use clap::{Arg, Command};

mod add;
mod fetch;
mod ls;

const NAME: &str = "hosts.rs";
const BIN_NAME: &str = "hosts";
const VERSION: &str = "0.0.1";

fn main() {
    let matches = Command::new(NAME)
        .bin_name(BIN_NAME)
        .version(VERSION)
        .about("一个可以订阅并快速合并生成hosts的工具")
        .arg_required_else_help(false)
        .subcommand_required(true)
        .subcommand(add::command())
        .subcommand(fetch::command())
        .subcommand(ls::command())
        .subcommand(Command::new("help").about("子命令帮助信息"))
        .arg(Arg::new("help").short('h').long("help").help("帮助信息"))
        .arg(
            Arg::new("version")
                .short('V')
                .long("version")
                .help("版本信息"),
        )
        .get_matches();

    match matches.subcommand() {
        Some((add::COMMAND_NAME, matches)) => add::handle(matches),
        Some((fetch::COMMAND_NAME, matches)) => fetch::handle(matches),
        Some((ls::COMMAND_NAME, matches)) => ls::handle(matches),
        _ => unreachable!(),
    };
}
