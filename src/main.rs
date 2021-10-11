use std::collections::HashMap;
use structopt::StructOpt;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = App::from_args();

    println!("{} - {:?} - {:?}", args.hostname, args.cmd, args.params);

    let body = reqwest::blocking::get(args.hostname)?.text()?;
    println!("{:#?}", body);
    Ok(())
}
