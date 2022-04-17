use clap::{Arg, ArgMatches, Command};
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;

pub const COMMAND_NAME: &str = "fetch";
const MIRROR_URL: &str = "https://ghproxy.com/"; // 使用的镜像地址
const DEFAULT_FILENAME: &str = "/etc/hosts";
const START_TAG: &str = "# hosts.rs";
const REPO_TAG: &str = "# repo: https://github.com/flxxyz/hosts.rs";

pub fn command() -> Command<'static> {
    let args_mirror = Arg::new("mirror")
        .short('m')
        .long("mirror")
        .help("使用镜像地址");
    let args_output = Arg::new("output")
        .short('o')
        .long("output")
        .help("直接输出内容");
    let args_filename = Arg::new("filename")
        .short('f')
        .long("filename")
        .takes_value(true)
        .help("指定输出的路径参数, 默认: /etc/hosts");
    let args_log = Arg::new("log").short('l').long("log").help("输出日志");
    let args_help = Arg::new("help").short('h').long("help").help("帮助信息");

    Command::new(COMMAND_NAME)
        .about("开始抓取hosts")
        .alias("f")
        .arg(args_mirror)
        .arg(args_output)
        .arg(args_filename)
        .arg(args_log)
        .arg(args_help)
}

pub fn handle(matches: &ArgMatches) -> &ArgMatches {
    let is_log = matches.is_present("log");

    let mut url_prefix = String::from("");
    if matches.is_present("mirror") {
        url_prefix = String::from(MIRROR_URL);
    }

    let mut responses = vec![format!("{}\n{}\n\n", START_TAG, REPO_TAG)];
    let urls = core::find();
    for url in urls.iter() {
        let res = reqwest::blocking::get(format!("{}{}", url_prefix, url));
        match res {
            Err(_) => {
                if is_log {
                    println!("抓取失败💩 {}", url);
                }
            }
            Ok(response) => {
                if response.status().is_success() {
                    if is_log {
                        println!("抓取成功🎉 {}", url);
                    }
                    responses.push(format!("# hosts link: {}\n{}\n\n", url, response.text().unwrap()));
                } else {
                    if is_log {
                        println!("抓取失败😭 {}", url);
                    }
                }
            }
        };
    }

    let mut hosts = core::merge("response", responses);
    if matches.is_present("output") {
        println!("{}", hosts);
    } else {
        let mut hosts_lines: Vec<&str> = hosts.split("\n").collect();
        let filename = matches.value_of("filename").unwrap_or(DEFAULT_FILENAME);
        let content = cat(filename).unwrap_or_default();

        let mut lines: Vec<&str> = vec![];

        if content == "" {
            lines = hosts_lines;
        } else {
            let content_lines: Vec<&str> = content.split("\n").collect();
            for line in content_lines.iter() {
                if line.starts_with(START_TAG) {
                    break;
                }
                lines.push(line);
            }
            lines.append(&mut hosts_lines);
        }

        hosts = lines.join("\n");

        match echo(hosts.as_str(), filename) {
            Ok(_) => {
                if is_log {
                    println!("写入成功🎉 {}", filename);
                }
            }
            Err(err) => {
                if is_log {
                    println!("写入失败😭 Err: {}", err);
                }
            }
        }
    }

    return matches;
}

fn cat(path: &str) -> io::Result<String> {
    let mut f = File::open(Path::new(path))?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn echo(s: &str, path: &str) -> io::Result<()> {
    let mut f = File::create(Path::new(path))?;
    f.write_all(s.as_bytes())
}
