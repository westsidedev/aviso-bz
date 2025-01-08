use std::{env::args, process::exit, sync::atomic::AtomicBool};

use avisobz::{Avisobz, Mode};
use colors::Colors;
use config::UserData;

pub mod avisobz;
pub mod browser;
pub mod colors;
pub mod config;

pub static GLOBAL_CONTROL: AtomicBool = AtomicBool::new(false);

#[tokio::main]
async fn main() {
    let mut arg: Vec<String> = args().collect();
    if arg.len() == 1 {
        msg_help().await;
    }
    let _ = arg.remove(0);
    match arg[0].as_str() {
        "--email" => {
            match arg[2].as_str() {
                "--passw" => (),
                _ => msg_help().await,
            }

            let mode = match arg[4].as_str() {
                "--YT" | "--yt" => Mode::YOUTUBE,
                "--SF" | "--sf" => Mode::SURFING,
                "--All" => Mode::ALL,
                _ => todo!(),
            };
            let _ = UserData::create(&arg[1], &arg[3], "", "").await;
            let mut headless = false;
            if arg.len() == 6 {
                if arg[5].contains("--headless") {
                    headless = true;
                }
            }
            Avisobz::start(mode, headless).await;
        }
        "--start" => {
            let mode = match arg[1].as_str() {
                "--YT" | "--yt" => Mode::YOUTUBE,
                "--SF" | "--sf" => Mode::SURFING,
                "--All" => Mode::ALL,
                _ => todo!(),
            };
            let mut headless = false;
            if arg.len() == 3 {
                if arg[2].contains("--headless") {
                    headless = true;
                }
            }
            Avisobz::start(mode, headless).await;
        }
        "--help" => msg_help().await,
        _ => msg_help().await,
    }
}

async fn msg_help() {
    let c = Colors::new().await;
    print!("\x1bc");
    println!("{}ARGUMENTS:{}", c.WHITE, c.CLOSE);
    println!(" --email      Email used for login in avisobz");
    println!(" --passw      Password used for login in avisobz");
    println!(" --start      Start software after first execution");
    println!(" --headless   Active headless mode (OPTIONAL)");
    println!(" --help       Show this message");
    println!(" --YT         Youtube mode");
    println!(" --SF         Surfing mode");
    println!(" --All        Youtube and surfing mode");
    println!();
    println!("{}USAGE:{}", c.WHITE, c.CLOSE);
    println!(" ./aviso-bz --email xxxx@xxxx --passw 123456 --YT --headless");
    println!(" ./aviso-bz --start --YT --headless");
    println!();
    println!("{}TELEGRAM:{}", c.WHITE, c.CLOSE);
    println!("Channel: https://t.me/earn_scripts");
    println!("Group: https://t.me/earn_scripts_group");
    exit(0);
}
