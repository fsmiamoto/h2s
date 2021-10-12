extern crate skim;
use skim::prelude::*;
use std::io::Cursor;
use structopt::StructOpt;
use regex::Regex;

#[derive(StructOpt)]
#[structopt(name = "http2shell", about = "shell2http back to the shell!")]
struct App {
    hostname: String,
    #[structopt(long, short, help = "command name to be executed")]
    cmd: Option<String>,
    #[structopt(
        long,
        short,
        help = "additional parameters required by command. e.g. 'pattern=error'"
    )]
    params: Option<String>,
}

fn do_request(url: &str) -> Result<reqwest::blocking::Response, reqwest::Error> {
    reqwest::blocking::get(url)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = App::from_args();

    println!("{} - {:?} - {:?}", args.hostname, args.cmd, args.params);

    let body = do_request(&args.hostname).unwrap().text().unwrap();

    let re2 = Regex::new(r"\s*<li><a[^>]*>/([^<]+)</a> <span[^>]*>- ([^>]+)<span>").unwrap();

    let mut list = String::new();

    // TODO: Add proper error handling
    for m in re2.captures_iter(&body) {
        let name = m.get(1).unwrap().as_str();
        let cmd = m.get(2).unwrap().as_str();
        list.push_str(format!("{}: {}\n", name, cmd).as_str());
    }

    let options = SkimOptionsBuilder::default()
        .height(Some("100%"))
        .multi(false)
        .build()
        .unwrap();

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(list));

    // `run_with` would read and show items from the stream
    let selected_items = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .expect("no selected option");

    for item in selected_items.iter() {
        print!("{}{}", item.output(), "\n");
    }

    Ok(())
}
