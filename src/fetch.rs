use clap::{Arg, ArgMatches, Command};

pub const COMMAND_NAME: &str = "fetch";
const MIRROR_URL: &str = "https://ghproxy.com/"; // 使用的镜像地址

pub fn command() -> Command<'static> {
    let args_mirror = Arg::new("mirror")
        .short('m')
        .long("mirror")
        .help("使用镜像地址");
    let args_help = Arg::new("help").short('h').long("help").help("帮助信息");

    Command::new(COMMAND_NAME)
        .about("开始抓取hosts")
        .alias("f")
        .arg(args_mirror)
        .arg(args_help)
}

pub fn handle(matches: &ArgMatches) -> &ArgMatches {
    let mut url_prefix = String::from("");
    if matches.is_present("mirror") {
        url_prefix = String::from(MIRROR_URL);
    }

    let urls = core::find();
    for url in urls.iter() {
        let res = reqwest::blocking::get(format!("{}{}", url_prefix, url));
        match res {
            Err(_) => {
                println!("抓取失败💩 {}", url);
            }
            Ok(response) => {
                if response.status().is_success() {
                    println!("抓取成功✌️ {}", url);
                    save(response.text().unwrap());
                } else {
                    println!("抓取失败💩 {}", url);
                }
            }
        };
    }

    merge_and_write_file();

    return matches;
}

fn save(_: String) {
    // TODO: 保存抓取的hosts
}

fn merge_and_write_file() {
    // TODO: 合并抓取到的hosts
    // TODO: 并写入本地hosts文件，可指定路径
    // TODO: 清理save()临时保存hosts
}
